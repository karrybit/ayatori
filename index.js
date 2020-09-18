import("./pkg").catch(console.error);

window.p = async function p() {
  const WASM = await import("./pkg");
  const j = await WASM.graph();
  console.log(j);
};

// import * as d3 from "d3";

// const data = {
//   nodes: [
//     { id: "お母さん" },
//     { id: "赤ずきんちゃん" },
//     { id: "おばあさん" },
//     { id: "オオカミ" },
//     { id: "猟師さん" },
//     { id: "お花" },
//   ],
//   links: [
//     { source: 0, target: 1 },
//     { source: 1, target: 2 },
//     { source: 3, target: 1 },
//     { source: 3, target: 2 },
//     { source: 4, target: 3 },
//     { source: 1, target: 5 },
//     { source: 5, target: 2 },
//   ],
// };

// const width = 3500;
// const height = 3500;

// const graph = {
//   console.log(data.nodes);
//   console.log(data.links);

//   const simulation = d3
//     .forceSimulation(data.nodes)
//     .force(
//       "link",
//       d3.forceLink(data.links).id((d) => d.id)
//     )
//     .force("charge", d3.forceManyBody())
//     .force("center", d3.forceCenter(width / 2, height / 2));

//   const svg = d3.create("svg").attr("viewBox", [0, 0, width, height]);

//   const link = svg
//     .append("g")
//     .attr("stroke", "#999")
//     .attr("stroke-opacity", 0.6)
//     .selectAll("line")
//     .data(data.links)
//     .join("line")
//     .attr("stroke-width", (d) => Math.sqrt(d.value));

//   const node = svg
//     .append("g")
//     .attr("stroke", "#fff")
//     .attr("stroke-width", 1.5)
//     .selectAll("circle")
//     .data(data.nodes)
//     .join("circle")
//     .attr("r", 5)
//     .attr("fill", color)
//     .call(drag(simulation));

//   node.append("title").text((d) => d.id);

//   simulation.on("tick", () => {
//     link
//       .attr("x1", (d) => d.source.x)
//       .attr("y1", (d) => d.source.y)
//       .attr("x2", (d) => d.source.x)
//       .attr("y2", (d) => d.source.y);

//     node.attr("cx", (d) => d.x).attr("cy", (d) => d.y);
//   });

//   invalidation.then(() => simulation.stop());

//   return svg.nodes();
// };
