/** @jsx jsx */

import { AppState } from "../app-state/AppState";
import { observer } from "mobx-react";
import * as React from "react";
import { HardeenGraphPath } from "../../../hardeen_wasm/pkg/hardeen_wasm";
import { SwitchToSubgraph, SwitchedToSubgraph } from "../app-state/Messenger";
import {css, jsx} from "@emotion/core";

interface GraphLevelMenuProps {
    appState: AppState
}

interface GraphLevelMenuState {
    graphPathStack: { path: HardeenGraphPath, displayName: string }[]
}

const MenuStyle = css`
    grid-area: graphLevelMenu;
    background-color: #222222;
    border-top: 1px solid black;
    padding: 1rem 2rem;
    color: white;
    ul {
        list-style: none;
        li {
            display: inline-block;
            cursor: pointer;
        }
    }
`;


class GraphLevelMenu extends React.PureComponent<GraphLevelMenuProps, GraphLevelMenuState> {

    constructor(props: GraphLevelMenuProps) {
        super(props);

        props.appState.messenger.subscribe("SwitchedToSubgraph", (message: SwitchedToSubgraph) => {
            this.setState( (oldState: GraphLevelMenuState) => ({
                graphPathStack: [...oldState.graphPathStack, { path: message.parentPath, displayName: message.displayName } ]
            }) );
        });

        this.state = {
            graphPathStack: [ { path: props.appState.hardeenCore.get_root_path(), displayName: "Root" } ]
        };
    }

    render() {
        return <div css={MenuStyle}>
            <ul>
            {
                this.state.graphPathStack.map( (graphPath, idx) => (
                    <li key={idx} onClick={() => {
                        this.props.appState.messenger.send({
                            type: "SwitchToGraphPath",
                            path: graphPath.path
                        });
                            this.setState( (oldState: GraphLevelMenuState) => ({
                                graphPathStack: oldState.graphPathStack.slice(0, idx+1)
                            }) );
                    }}> {idx!=0 && "→"} {graphPath.displayName} </li>
                ))
            }
            </ul>
        </div>
    }

}

export default observer(GraphLevelMenu);