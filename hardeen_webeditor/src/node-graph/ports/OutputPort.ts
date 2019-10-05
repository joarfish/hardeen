import {
    LinkModel,
    PortModel,
    PortModelAlignment,
    PortModelGenerics,
    PortModelOptions
} from '@projectstorm/react-diagrams';
import {DefaultLinkModel} from '@projectstorm/react-diagrams';
import { AbstractModelFactory, DeserializeEvent } from '@projectstorm/react-canvas-core';

export interface OutputPortOptions extends PortModelOptions {

}

export interface OutputPortGenerics extends PortModelGenerics {
    OPTIONS: OutputPortOptions;
}

export class OutputPort extends PortModel<OutputPortGenerics> {

    constructor(options: OutputPortOptions);
    constructor(options: OutputPortOptions | boolean, name?: string) {

        options = options as OutputPortOptions;
        super({
            alignment: PortModelAlignment.BOTTOM,
            type: 'output-port',
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
            return false;
        }
        else {
            return port.canLinkToPort(this);
        }
    }

    createLinkModel(factory?: AbstractModelFactory<LinkModel>): LinkModel {
        let link = super.createLinkModel();
        if (!link && factory) {
            return factory.generateModel({});
        }
        return link || new DefaultLinkModel();
    }
}