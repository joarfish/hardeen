import { HardeenNodeModel } from "../node-graph/nodes/HardeenNodeModel";
import { HardeenGraphPath } from "../../../hardeen_wasm/pkg/hardeen_wasm";
import { AppState } from "../app-state/AppState";
import { SwitchToSubgraph, SwitchToGraphPath } from "../app-state/Messenger";
import { DiagramModel, DiagramEngine } from "@projectstorm/react-diagrams";

interface GraphState {
    outputNode: HardeenNodeModel,
    model: Object,
}

export const trackGraphStates = (model: DiagramModel, engine: DiagramEngine, appState: AppState) => {
    let graphStates : Map<string, GraphState> = new Map();

    const switchToGraphPath = (graph_path: HardeenGraphPath) => {

        const hc = appState.hardeenCore;
        let parentPath = appState.currentGraphPath;

        const lastModel = {
			outputNode: appState.outputNode,
			model: model.serialize()
		};

		graphStates.set(hc.hash_graph_path(parentPath), lastModel);

		appState.currentGraphPath = graph_path;
		let subgraphState = graphStates.get(hc.hash_graph_path(graph_path));

		if(subgraphState!=undefined) {
			model.deserializeModel(subgraphState.model as any, engine);
		}
		else {
			subgraphState = {
				outputNode: null,
				model: new DiagramModel()
			}
			
			model.deserializeModel((subgraphState.model as DiagramModel).serialize(), engine);
			graphStates.set(hc.hash_graph_path(graph_path), subgraphState);
        }

		appState.selectedNode = null;

        appState.messenger.send({
            type: "SetOutputNode",
            node: subgraphState.outputNode
        });

		engine.repaintCanvas();   
    }

    appState.messenger.subscribe("SwitchToSubgraph", (message: SwitchToSubgraph) => {

        const hc = appState.hardeenCore;
        let parentPath = appState.currentGraphPath;

        const subgraphPath = hc.get_graph_path(parentPath, message.node.getHardeenHandle())

        switchToGraphPath(subgraphPath);

        appState.messenger.send({
            type: "SwitchedToSubgraph",
            parentPath: subgraphPath,
            displayName: message.node.getHardeenHandle().get_node_type()
        });
    });


    appState.messenger.subscribe("SwitchToGraphPath", (message: SwitchToGraphPath) => {

        switchToGraphPath(message.path);

    });

}