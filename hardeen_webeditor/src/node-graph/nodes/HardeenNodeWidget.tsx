/** @jsx jsx */

import * as React from 'react';
import {DiagramEngine, PortModelAlignment, PortWidget} from '@projectstorm/react-diagrams-core';
import {HardeenNodeModel} from './HardeenNodeModel';
import {css, jsx} from "@emotion/core";
import Messenger from '../app-state/Messenger';

export interface HardeenNodeWidgetProps {
	node: HardeenNodeModel;
	engine: DiagramEngine;
	messenger: Messenger;
}

export interface HardeenNodeWidgetState {}

const NodeStyleSelected = css`
	border-color: mediumpurple;
	box-shadow: 0 0 10px mediumpurple;
`;

const NodeStyleBase = css`
    border: solid 2px gray;
    border-radius: 5px;
    min-width: 150px;
    min-height: 50px;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    position: relative;
    flex-direction: column;
	background-color: #222222;
	color: white;
	:hover {
		${NodeStyleSelected}
	}
`;


const PortStyle = css`
    width: 12px;
    height: 12px;
    margin: 2px;
    border-radius: 4px;
    background: darkgray;
	cursor: pointer;
	:hover {
		background: mediumpurple;
	}
`;

const PortsContainerStyle = css`
	display: flex;
	width: 100%;
	flex-direction: row;
	justify-content: space-around;
`;

const TextStyle = css`
	width: 100%;
	text-align: center;
	box-sizing: border-box;
`;

const EyeIcon = css`
	width: 1.5rem;
	margin: auto 0;
`;


export class HardeenNodeWidget extends React.Component<HardeenNodeWidgetProps, HardeenNodeWidgetState> {
	constructor(props: HardeenNodeWidgetProps) {
		super(props);
		this.state = {};
	}

	render() {

		const nodeStyle = css`
			${NodeStyleBase}
			${this.props.node.isSelected() && NodeStyleSelected}
		`;

		const fillColor = this.props.node.isOutputNode ? "green" : "white";

		return (
			<div css={nodeStyle} onClick={this.handleClick} onDoubleClick={this.handleDoubleClick}>
				<div css={css`display: flex; flex-direction: row; width: 100%; padding: 0.25rem;`}>
					<div css={EyeIcon} onClick={this.makeOutputNode} >
						<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512">
							<path fill={fillColor} d="M572.52 241.4C518.29 135.59 410.93 64 288 64S57.68 135.64 3.48 241.41a32.35 32.35 0 0 0 0 29.19C57.71 376.41 165.07 448 288 448s230.32-71.64 284.52-177.41a32.35 32.35 0 0 0 0-29.19zM288 400a144 144 0 1 1 144-144 143.93 143.93 0 0 1-144 144zm0-240a95.31 95.31 0 0 0-25.31 3.79 47.85 47.85 0 0 1-66.9 66.9A95.78 95.78 0 1 0 288 160z" />
						</svg>
					</div>
					<div css={css`display: flex; flex-direction: column; margin:auto;`}>
						<div css={PortsContainerStyle}>
						{
							Object.values(this.props.node.getPorts()).filter( (port) => port.getOptions().alignment==PortModelAlignment.TOP ).map( (port) => (
									<PortWidget key={port.getID()} engine={this.props.engine} port={port}>
										<div css={PortStyle} />
									</PortWidget>
							))
						}
						</div>
						<div css={TextStyle}><p>{this.props.node.typeName}</p></div>
						<div css={PortsContainerStyle}>
							<PortWidget engine={this.props.engine} port={this.props.node.getPort('out')}>
								<div css={PortStyle} />
							</PortWidget>
						</div>
					</div>
					<div css={css`width: 1.5rem;`} />
				</div>
			</div>
		);
	}

	makeOutputNode = () => {
		this.props.messenger.send({
			type: "SetOutputNode",
			node: this.props.node
		});
	}

	handleClick = () => {
		this.props.messenger.send({
			type: "NodeSelected",
			node: this.props.node
		});
	}

	handleDoubleClick = () => {
		if(this.props.node.isSubgraphProcessor) {
			this.props.messenger.send({
				type: "SubgraphNodeSelected",
				node: this.props.node
			});
		}
	}
}
