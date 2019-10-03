import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './main.css';
import createEngine, { DiagramModel, DefaultDiagramState } from '@projectstorm/react-diagrams';
import { HardeenNodeFactory } from './hardeen-nodes/HardeenNodeFactory';
import { HardeenNodeModel } from './hardeen-nodes/HardeenNodeModel';
import { HardeenWebeditor } from './HardeenWebeditor';
import {AppState} from "./app-state/AppState";
import Messenger, {CreateNode, SaveAll, CreateLink, DeleteLink, SetOutputNode, NodeSelected, DeleteNode, SubgraphNodeSelected} from "./app-state/Messenger";
import {action} from "mobx";
import {SlottedInputPort} from "./hardeen-nodes/ports/SlottedInputPort";
import {MultipleInputPort} from "./hardeen-nodes/ports/MultipleInputPort";
import PortFactory from './hardeen-nodes/ports/PortFactory';
import { HardeenGraphPath } from '../../hardeen_wasm/pkg/hardeen_wasm';


import(/* webpackChunkName: "hardeen" */ "../../hardeen_wasm/pkg" ).then( (hardeen) => {

	const appState = new AppState();

	appState.messenger = new Messenger();
	appState.hardeenCore = hardeen.HardeenCoreInterface.new();
	appState.allNodeTypes = appState.hardeenCore.get_node_types();

	console.log("Node Types:");
	console.log(appState.allNodeTypes);

	const engine = createEngine();

	engine.getNodeFactories().registerFactory(new HardeenNodeFactory(appState.messenger));
	engine.getPortFactories().registerFactory(new PortFactory('output-port'));
	engine.getPortFactories().registerFactory(new PortFactory('input-port-slotted'));
	engine.getPortFactories().registerFactory(new PortFactory('input-port-multiple'));

	
	const state = engine.getStateMachine().getCurrentState();
	if (state instanceof DefaultDiagramState) {
		state.dragNewLink.config.allowLooseLinks = false;
	}

	const model = new DiagramModel();

	appState.currentGraphPath = appState.hardeenCore.get_root_path();

	const handle = appState.hardeenCore.add_processor_node(appState.currentGraphPath,"Empty");
	const node = new HardeenNodeModel({hardeenHandle: handle, nodeType: appState.allNodeTypes[0], isSubgraphProcessor: false});
	node.setPosition(50,50);
	model.addNode(node);

	engine.setModel(model);

	model.registerListener({
		nodesUpdated: action((event) => {
			if(!event.isCreated && event.node instanceof HardeenNodeModel) {
				appState.messenger.send({
					type: "DeleteNode",
					hdNodeHandle: event.node.hardeenHandle
				});
			}
		} ),
		eventDidFire: action((event) => {

			// @ts-ignore
			if(event.function == "linksUpdated" && event.isCreated && event.link.targetPort==null) {
				// @ts-ignore
				event.link.registerListener({
					targetPortChanged: action(linkEvent => {
						let msg = {
							type: "CreateLink",
							// @ts-ignore
							from: linkEvent.entity.sourcePort.getNode().getHardeenHandle(),
							// @ts-ignore
							to: linkEvent.entity.targetPort.getNode().getHardeenHandle()
						};
						// @ts-ignore
						if(event.link.targetPort instanceof SlottedInputPort) {
							// @ts-ignore
							msg.slot = event.link.targetPort.getOptions().slotNumber;
						}

						appState.messenger.send(msg);
					})
				});
			}
			// @ts-ignore
			else if(event.function == "linksUpdated" && !event.isCreated && event.link.targetPort!=null) {
				let msg : DeleteLink = {
					type: "DeleteLink",
					// @ts-ignore
					from: event.link.sourcePort.getNode().getHardeenHandle(),
					// @ts-ignore
					to: event.link.targetPort.getNode().getHardeenHandle()
				};

				// @ts-ignore
				if(event.link.targetPort instanceof SlottedInputPort) {
					// @ts-ignore
					msg.slot = event.link.targetPort.getOptions().slotNumber;
				}

				appState.messenger.send(msg);
			}
		})
	});

	appState.messenger.subscribe("CreateNode", (message: CreateNode) => {
		const handle = appState.hardeenCore.add_processor_node(appState.currentGraphPath,message.nodeType.name);
		const isSubgraphProcessor = appState.hardeenCore.is_node_subgraph_processor(appState.currentGraphPath, handle);

		console.log("Node Created!");

		const node = new HardeenNodeModel({hardeenHandle: handle, nodeType: message.nodeType, isSubgraphProcessor: isSubgraphProcessor});
		node.setPosition(50,50);
		model.addNode(node);
		engine.repaintCanvas();
		appState.messenger.send({type: "NodeCreated", hdNodeHandle: handle});
	});

	appState.messenger.subscribe("DeleteNode", (message: DeleteNode) => {
		console.log("Delete Node!");
		if(message.hdNodeHandle == appState.selectedNode) {
			appState.selectedNode = null;
		}

		appState.hardeenCore.remove_node(appState.currentGraphPath,message.hdNodeHandle);
	});

	appState.messenger.subscribe("CreateLink", (message: CreateLink) => {
		if(message.slot != undefined) {
			appState.hardeenCore.connect_nodes_slotted(appState.currentGraphPath,message.from, message.to, message.slot);
			console.log("Connected Slotted");
		} else {
			appState.hardeenCore.connect_nodes(appState.currentGraphPath,message.from, message.to);
			console.log("Connected Multiple");
		}
	});

	appState.messenger.subscribe("DeleteLink", (message: DeleteLink) => {

		if(message.to.ptr == 0 || message.from.ptr == 0) {
			return;
		}

		if(message.hasOwnProperty("slot")) {

			appState.hardeenCore.disconnect_nodes_slotted(appState.currentGraphPath,message.from, message.to, message.slot);
			console.log("Delete Slotted");
		} else {
			appState.hardeenCore.disconnect_nodes(appState.currentGraphPath,message.from, message.to);
			console.log("Delete Multiple");
		}
	});

	let savedModel = null;

	appState.messenger.subscribe("SaveAll", (message: SaveAll) => {
		const serializedModel = model.serialize();
		console.log(serializedModel);

		if(savedModel!=null) {
			model.deserializeModel(savedModel, engine);
		}

		savedModel = serializedModel;
	});

	appState.messenger.subscribe("SetOutputNode", (message: SetOutputNode) => {
		console.log("Set output Node!");

		if(appState.outputNode) {
			appState.outputNode.isOutputNode = false;
		}
		
		message.node.isOutputNode = true;
		appState.outputNode = message.node;

		console.log(message.node.getHardeenHandle());

		appState.hardeenCore.set_output_node(appState.currentGraphPath,message.node.getHardeenHandle());
		appState.renderOutput = appState.hardeenCore.run_processors(appState.currentGraphPath);
	});

	appState.messenger.subscribe("NodeSelected", (message: NodeSelected) => {
		console.log("Node Selected!!");
		appState.selectedNode = message.node.getHardeenHandle();
	});

	let graphStates : Map<HardeenGraphPath, Object> = new Map();
	let parentState : HardeenGraphPath = null;

	appState.messenger.subscribe("SubgraphNodeSelected", (message: SubgraphNodeSelected) => {
		console.log("Subgraph Node Selected!!");
		const subgraphPath = appState.hardeenCore.get_graph_path(appState.currentGraphPath, message.node.hardeenHandle);
		parentState = appState.currentGraphPath;

		const lastModel = model.serialize();
		graphStates.set(appState.currentGraphPath, lastModel);

		appState.currentGraphPath = subgraphPath;
		let subgraphModel = graphStates.get(appState.currentGraphPath);

		if(subgraphModel) {
			model.deserializeModel(subgraphModel as any, engine);
		}
		else {
			subgraphModel = new DiagramModel();
			
			model.deserializeModel((subgraphModel as DiagramModel).serialize(), engine);
			graphStates.set(appState.currentGraphPath, (subgraphModel as DiagramModel).serialize());
		}
		engine.repaintCanvas();
	});

	appState.messenger.subscribe("MoveLevelUp", (message) => {
		let subgraphModel = model.serialize();
		graphStates.set(appState.currentGraphPath, subgraphModel);

		let parentModel = graphStates.get(parentState);

		model.deserializeModel(parentModel as any, engine);
		engine.repaintCanvas();
	});

	ReactDOM.render(<HardeenWebeditor engine={engine} appState={appState} />, document.querySelector('#application'));

});


