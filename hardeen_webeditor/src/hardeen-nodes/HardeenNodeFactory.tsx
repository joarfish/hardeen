import * as React from 'react';
import { HardeenNodeModel } from './HardeenNodeModel';
import { HardeenNodeWidget } from './HardeenNodeWidget';
import { AbstractReactFactory } from '@projectstorm/react-canvas-core';
import { DiagramEngine } from '@projectstorm/react-diagrams-core';
import Messenger from '../app-state/Messenger';

export class HardeenNodeFactory extends AbstractReactFactory<HardeenNodeModel, DiagramEngine> {

	messenger : Messenger;

	constructor(messenger: Messenger) {
		super('hardeen-node');

		this.messenger = messenger;
	}

	generateModel(event) {
		return new HardeenNodeModel(event.initialConfig);
	}

	generateReactWidget(event): JSX.Element {
		return <HardeenNodeWidget engine={this.engine as DiagramEngine} node={event.model} messenger={this.messenger} />;
	}
}