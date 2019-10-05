import { AppState } from "../app-state/AppState";
import { HardeenGraphPath } from '../../../hardeen_wasm/pkg/hardeen_wasm';
import {CreateNode, SaveAll, CreateLink, DeleteLink, SetOutputNode, NodeSelected, DeleteNode, SubgraphNodeSelected, RunProcessors} from "../app-state/Messenger";
import { HardeenNodeModel } from '../node-graph/nodes/HardeenNodeModel';
import { DiagramEngine, DiagramModel } from "@projectstorm/react-diagrams";

export const registerMessageHandler = (model: DiagramModel, engine: DiagramEngine, appState: AppState) => {
    
	appState.messenger.subscribe("CreateNode", (message: CreateNode) => {
		const handle = appState.hardeenCore.add_processor_node(appState.currentGraphPath,message.nodeType.name);
		const isSubgraphProcessor = appState.hardeenCore.is_node_subgraph_processor(appState.currentGraphPath, handle);

		const node = new HardeenNodeModel({hardeenHandle: handle, nodeType: message.nodeType, isSubgraphProcessor: isSubgraphProcessor});
		node.setPosition(50,50);
		model.addNode(node);
		engine.repaintCanvas();
		appState.messenger.send({type: "NodeCreated", hdNodeHandle: handle});
	});

	appState.messenger.subscribe("DeleteNode", (message: DeleteNode) => {
		if(message.hdNodeHandle == appState.selectedNode) {
			appState.selectedNode = null;
		}

		appState.hardeenCore.remove_node(appState.currentGraphPath,message.hdNodeHandle);
	});

	appState.messenger.subscribe("CreateLink", (message: CreateLink) => {
		if(message.slot != undefined) {
			appState.hardeenCore.connect_nodes_slotted(appState.currentGraphPath,message.from, message.to, message.slot);
		} else {
			appState.hardeenCore.connect_nodes(appState.currentGraphPath,message.from, message.to);
		}
	});

	appState.messenger.subscribe("DeleteLink", (message: DeleteLink) => {

		if(message.to.ptr == 0 || message.from.ptr == 0) {
			return;
		}

		if(message.hasOwnProperty("slot")) {

			appState.hardeenCore.disconnect_nodes_slotted(appState.currentGraphPath,message.from, message.to, message.slot);
		} else {
			appState.hardeenCore.disconnect_nodes(appState.currentGraphPath,message.from, message.to);
		}
	});

	let savedModel = null;

	appState.messenger.subscribe("SaveAll", (message: SaveAll) => {
		const serializedModel = model.serialize();

		if(savedModel!=null) {
			model.deserializeModel(savedModel, engine);
		}

		savedModel = serializedModel;
	});

	appState.messenger.subscribe("SetOutputNode", (message: SetOutputNode) => {
		if(appState.outputNode) {
			appState.outputNode.isOutputNode = false;
        }
        
        appState.outputNode = message.node;
        
        if(message.node!=null) {
            message.node.isOutputNode = true;
            appState.messenger.send({type: "RunProcessors"});
        }

		engine.repaintCanvas();
    });
    
    appState.messenger.subscribe("RunProcessors", (message: RunProcessors) => {
 
        appState.hardeenCore.set_output_node(appState.currentGraphPath,appState.outputNode.getHardeenHandle());
        const result = appState.hardeenCore.run_processors(appState.currentGraphPath);

        if(result!="No result") {
            appState.renderOutput = appState.hardeenCore.run_processors(appState.currentGraphPath);
        }
	});

	appState.messenger.subscribe("NodeSelected", (message: NodeSelected) => {
		appState.selectedNode = message.node.getHardeenHandle();
	});

	appState.messenger.subscribe("SubgraphNodeSelected", (message: SubgraphNodeSelected) => {
        appState.messenger.send({
            type: "SwitchToSubgraph",
            node: message.node
        });
	});

	appState.messenger.subscribe("MoveLevelUp", (message) => {
        appState.messenger.send({
            type: "SwitchToGraphPath",
            path: appState.hardeenCore.get_root_path()
        })
	});
}