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

export interface MultipleInputPortOptions extends PortModelOptions {

}

export interface MultipleInputPortGenerics extends PortModelGenerics {
    OPTIONS: MultipleInputPortOptions;
}

export class MultipleInputPort extends PortModel<MultipleInputPortGenerics> {

    constructor(options: MultipleInputPortOptions);
    constructor(options: MultipleInputPortOptions | boolean, name?: string) {

        options = options as MultipleInputPortOptions;
        super({
            alignment: PortModelAlignment.TOP,
            type: 'input-port-multiple',
            ...options
        });

    }

    deserialize(event: DeserializeEvent<this>) {
        super.deserialize(event);
    }

    serialize() {
        return {
            ...super.serialize(),
        };
    }

    canLinkToPort(port: PortModel): boolean {

        if(port instanceof OutputPort) {
            return true;
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