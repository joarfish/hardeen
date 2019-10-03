import {NodeModel} from '@projectstorm/react-diagrams';
import {BaseModelOptions} from '@projectstorm/react-canvas-core';
import {HardeenHandle, NodeType} from '../../../hardeen_wasm/pkg';
import {OutputPort} from "./ports/OutputPort";
import {MultipleInputPort} from "./ports/MultipleInputPort";
import {SlottedInputPort} from "./ports/SlottedInputPort";

export interface HardeenNodeModelOptions extends BaseModelOptions {
	hardeenHandle: HardeenHandle;
	nodeType: NodeType;
	isSubgraphProcessor: boolean,
}

export class HardeenNodeModel extends NodeModel {
	typeName: string;
	nodeType: NodeType;
	hardeenHandle: HardeenHandle;
	isOutputNode: boolean;
	isSubgraphProcessor: boolean;

	constructor(options: HardeenNodeModelOptions) {
		super({
			...options,
			type: 'hardeen-node'
		});
		this.typeName = options.nodeType.name;
		this.nodeType = options.nodeType;
		this.hardeenHandle = options.hardeenHandle;
		this.isOutputNode = false;
		this.isSubgraphProcessor = options.isSubgraphProcessor;

		const input_type = this.nodeType.input_type;

		if (input_type.type == "Slotted") {
			for (let i = 0; i < input_type.number_of_slots; i++) {
				this.addPort(
					new SlottedInputPort({maximumLinks: 1, name: "in" + i, slotNumber: i})
				);
			}
		} else if (input_type.type == "Multiple") {
			this.addPort(
				new MultipleInputPort({name: "in"})
			)
		}

		this.addPort(
			new OutputPort({
				name: 'out',
			})
		);
	}

	getHardeenHandle() : HardeenHandle {
		return this.hardeenHandle;
	}

	serialize() {
		return {
			...super.serialize(),
			typeName: this.typeName,
			nodeType: this.nodeType,
			hardeenHandle: this.hardeenHandle,
			isOutputNode: this.isOutputNode,
			isSubgraphProcessor: this.isSubgraphProcessor
		};
	}

	deserialize(event): void {
		super.deserialize(event);
		this.typeName = event.data.typeName;
		this.nodeType = event.data.nodeType;
		this.hardeenHandle = event.data.hardeenHandle;
		this.isOutputNode = event.data.isOutputNode;
		this.isSubgraphProcessor = event.data.isSubgraphProcessor;
	}
}