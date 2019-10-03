/** @jsx jsx */

import {Point, Shape, Position, HardeenHandle} from "../../../hardeen_wasm/pkg";
import {AppState} from "../app-state/AppState";
import * as React from "react";
import {css, jsx, SerializedStyles} from "@emotion/core";
import {observer} from "mobx-react";

interface RenderViewProps {
    appState: AppState,
}

type Viewbox = {
    x: number,
    y: number,
    width: number,
    height: number,
    baseWidth: number
}

type ViewState = { type: "IDLE" } | { type: "MOVING", startX: number, startY: number };

interface RenderViewState {
    viewbox: Viewbox,
}

class RenderView extends React.PureComponent<RenderViewProps, RenderViewState> {

    svgRootRef : React.RefObject<SVGSVGElement>;
    containerRef: React.RefObject<HTMLDivElement>;

    tmpViewBox : Viewbox;
    viewState: ViewState;

    reactSyncTimer : number;

    constructor(props: RenderViewProps) {
        super(props);

        this.state = {
            viewbox: { x: 0, y: 0, width: undefined, height: undefined, baseWidth: 750 }
        }

        this.viewState = { type: "IDLE" };

        this.svgRootRef = React.createRef();
        this.containerRef = React.createRef();
        this.tmpViewBox = null;
    }

    componentDidMount() {
        const rect = this.containerRef.current.getBoundingClientRect();
        this.setState( (prevState: RenderViewState) => ({
            viewbox: { ...prevState.viewbox, width: rect.width, height: rect.height }
        }));
    }

    render() {
        
        return <div ref={this.containerRef} css={css`grid-area: viewport`}>
            { this.state.viewbox.width && this.renderSvg() }
            </div>;
    }

    renderSvg() {
        const viewbox = this.state.viewbox;
        const world = this.props.appState.renderOutput;

        return <svg ref={this.svgRootRef}
                viewBox={`${viewbox.x} ${viewbox.y} ${viewbox.baseWidth} ${viewbox.baseWidth * viewbox.height / viewbox.width} `}
                width={viewbox.width}
                height={viewbox.height}
                onMouseDown={this.handleMouseDown}
                onMouseUp={this.handleMouseUp}
                onMouseMove={this.handleMouseMove}
                onWheel={this.handleWheel}>
            {
                world && Object.entries(world.shapes).map( (entry) => 
                    <path key={entry[0]} d={this.getPathStringForShape(entry[1], world.points)} stroke="black" fill="transparent" /> )
            }
            {
                world && Object.entries(world.points).map( (entry) =>
                    <circle key={entry[0]} cx={entry[1].position[0]} cy={entry[1].position[1]} r="1" fill="red" />
                )
            }
        </svg>
    }

    getPathStringForShape = (shape: Shape, points: {[handle: number]: Point}) => {

        if(Object.keys(shape.vertices).length == 0) return;

        const first_point_index = shape.vertices[0].index;
        const first_point = points[first_point_index];

        let pathString : string = `M ${first_point.position[0]} ${first_point.position[1]} `;
        let lastOutTangent : Position = first_point.out_tangent;

        for(let vertex_nr = 1; vertex_nr < Object.keys(shape.vertices).length; vertex_nr++) {
            const p_index = shape.vertices[vertex_nr].index;
            const p = points[p_index];

            pathString += this.getPointSegment(p, lastOutTangent);

            lastOutTangent = p.out_tangent;
        }

        if(shape.closed) {
            pathString += this.getPointSegment(first_point, lastOutTangent);
        }

        return pathString;
    }

    getPointSegment(point: Point, lastOutTangent: Position) {
        let segmentString = "";
        if(this.isPositionNull(lastOutTangent)) {
            if(this.isPositionNull(point.in_tangent)) {
                segmentString += "L ";
            }
            else {
                segmentString += "Q "+point.in_tangent[0]+" "+point.in_tangent[1]+" ";
            }
        }
        else {
            if(this.isPositionNull(point.in_tangent)) {
                segmentString += "Q "+lastOutTangent[0]+" "+lastOutTangent[1]+" ";
            }
            else {
                segmentString += "C "+lastOutTangent[0]+" "+lastOutTangent[1]+" "+point.in_tangent[0]+" "+point.in_tangent[1]+" ";
            }
        }

        segmentString += point.position[0]+" "+point.position[1]+" ";
        return segmentString;
    }

    isPositionNull(p: Position) : boolean {
        return p[0] === 0 && p[1] === 0;
    }

    updateSvgSize = () => {
        const rect = this.containerRef.current.getBoundingClientRect();
        this.setState( (prevState: RenderViewState) => ({
            viewbox: { ...prevState.viewbox, width: rect.width, height: rect.height }
        }));
    }

    handleMouseDown = (event) => {
        if(this.viewState.type=="IDLE") {

            const mousePosition = this.convertToSvgPosition(event.clientX, event.clientY);

            this.svgRootRef.current.style.cursor = "move";

            this.viewState = {type: "MOVING", startX: mousePosition.x, startY: mousePosition.y};
           
            this.tmpViewBox = { ...this.state.viewbox };
            window.requestAnimationFrame(this.renderMouseMoveFrame);
        }
    };

    handleMouseUp = (event) => {
        if(this.viewState.type == "MOVING") {
            this.svgRootRef.current.style.cursor = "unset";
            const mousePosition = this.convertToSvgPosition(event.clientX, event.clientY);
            const dX = this.viewState.startX - mousePosition.x;
            const dY = this.viewState.startY - mousePosition.y;

            this.setState( (oldState: RenderViewState) => {
                return {
                    viewbox: {...oldState.viewbox, x: this.tmpViewBox.x, y: this.tmpViewBox.y},
                }
            });

            this.viewState = { type: "IDLE" };
        }
    };

    handleMouseMove = (event) => {
        if(this.viewState.type == "MOVING") {
            
            let mousePosition = this.convertToSvgPosition(event.clientX, event.clientY);
            const x = this.viewState.startX;
            const y = this.viewState.startY;

            this.tmpViewBox.x += x - mousePosition.x;
            this.tmpViewBox.y += y - mousePosition.y;
        }
    };

    handleWheel = (event) => {
        window.clearTimeout(this.reactSyncTimer);

        event.stopPropagation();
        const mousePosition = this.convertToSvgPosition(event.clientX, event.clientY);

        this.tmpViewBox.baseWidth +=  10 * Math.sign(event.deltaY);

        this.updateViewbox();

        this.reactSyncTimer = window.setTimeout(this.synReactState, 250);
    };

    synReactState = () => {
        this.setState({
            viewbox: {...this.tmpViewBox},
        });
    }

    private renderMouseMoveFrame = () => {
        if(this.viewState.type == "MOVING") {
            this.updateViewbox();
            window.requestAnimationFrame(this.renderMouseMoveFrame);
        }
    }

    private updateViewbox() {
        const viewbox = this.tmpViewBox;
        const ratio = viewbox.height / viewbox.width;

        this.svgRootRef.current.setAttribute('viewBox',
            `${viewbox.x} ${viewbox.y} ${viewbox.baseWidth} ${viewbox.baseWidth*ratio}`);
    }

    private convertToSvgPosition = (x: number, y: number) => {
        let svgCTM = this.svgRootRef.current.getScreenCTM();
        let pt = this.svgRootRef.current.createSVGPoint();

        pt.x = x;
        pt.y = y;

        let cursorPt = pt.matrixTransform(svgCTM.inverse());
        return {x: cursorPt.x , y: cursorPt.y}
    }
}

export default observer(RenderView);