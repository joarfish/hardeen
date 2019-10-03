/** @jsx jsx */

import * as React from 'react';
import { DiagramEngine } from '@projectstorm/react-diagrams';
import { CanvasWidget } from '@projectstorm/react-canvas-core';
import {css, jsx} from "@emotion/core";
import EditorMenu from "./components/EditorMenu";
import {AppState} from "./app-state/AppState";
import RenderView from "./components/RenderView";
import PropertyEditor from './components/PropertyEditor';

export interface HardeenWebeditorProps {
	engine: DiagramEngine;
	appState: AppState
}

const canvasStyle = css`
	background: #333333;
	grid-area: graphEditor;
`;

const hardeenditorStyle = css`
	position: relative;
	height: 100%;
	width: 100%;
	display: grid;
	grid-template-areas: 	"graphMenu graphMenu graphMenu"
							"graphEditor verticalSeperator viewport";
	grid-template-rows: 4rem calc(100vh - 4rem);
	grid-template-columns: calc(50vw - 0.125rem) 0.25rem calc(50vw - 0.125rem);
`;

const verticalSeperator = css`
	grid-area: verticalSeperator;
	background-color: black;
`;



export class HardeenWebeditor extends React.Component<HardeenWebeditorProps> {
	render() {
		return <div css={hardeenditorStyle}>
			<EditorMenu nodeTypes={this.props.appState.allNodeTypes} messenger={this.props.appState.messenger} />
			<CanvasWidget css={canvasStyle} engine={this.props.engine} />
			<div css={verticalSeperator} />
			<RenderView appState={this.props.appState} />
			<PropertyEditor appState={this.props.appState} />
		</div>;
	}
}