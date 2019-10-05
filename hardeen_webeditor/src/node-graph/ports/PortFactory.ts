import { AbstractModelFactory } from "@projectstorm/react-canvas-core";
import { DiagramEngine } from '@projectstorm/react-diagrams-core';
import { OutputPort } from "./OutputPort";
import { SlottedInputPort } from "./SlottedInputPort";
import { MultipleInputPort } from "./MultipleInputPort";


class PortFactory extends AbstractModelFactory<OutputPort, DiagramEngine> {
    type : string;

    constructor(type: "output-port" | "input-port-slotted" | "input-port-multiple") {
        super(type);
        this.type = type;
    }

    generateModel(event) {

        if(this.type=="output-port") {
            return new OutputPort(event.initialConfig);
        }
        else if(this.type=="input-port-slotted") {
            return new SlottedInputPort(event.initialConfig);
        }
        else if(this.type=="input-port-multiple") {
            return new MultipleInputPort(event.initialConfig);
        }
        
    }
}

export default PortFactory;