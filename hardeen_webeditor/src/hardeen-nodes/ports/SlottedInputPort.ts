import {
    LinkModel,
    PortModel,
    PortModelAlignment,
    PortModelGenerics,
    PortModelOptions
} from '@projectstorm/react-diagrams';
import {DefaultLinkModel} from '@projectstorm/react-diagrams';
import { AbstractModelFactory, DeserializeEvent } from '@projectstorm/react-canvas-core';
import {OutputPort} from "./OutputPort";

export interface SlottedInputPortOptions extends PortModelOptions {
    slotNumber: number;
}

export interface SlottedInputPortGenerics extends PortModelGenerics {
    OPTIONS: SlottedInputPortOptions;
}

export class SlottedInputPort extends PortModel<SlottedInputPortGenerics> {

    constructor(options: SlottedInputPortOptions);
    constructor(options: SlottedInputPortOptions | boolean) {

        options = options as SlottedInputPortOptions;
        super({
            alignment: PortModelAlignment.TOP,
            type: 'input-port-slotted',
            ...options
        });

    }

    deserialize(event: DeserializeEvent<this>) {
        super.deserialize(event);
        this.options.slotNumber = event.data.slotNumber;
    }

    serialize() {
        return {
            ...super.serialize(),
            slotNumber: this.options.slotNumber
        };
    }

    canLinkToPort(port: PortModel): boolean {

        if(port instanceof OutputPort) {
            if(this.getOptions().maximumLinks > Object.values(this.getLinks()).length) {
                return true;
            }
        }
        return false;
    }

    createLinkModel(factory?: AbstractModelFactory<LinkModel>): LinkModel {
        let link = super.createLinkModel();
        if (!link && factory) {
            return factory.generateModel({});
        }
        return link || new DefaultLinkModel();
    }
}