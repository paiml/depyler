import React, { useEffect, useRef } from "react";
import * as d3 from "d3";
import { ExecutionResult } from "@/types";

interface PerformanceChartProps {
  executionResult: ExecutionResult | null;
  width?: number;
  height?: number;
}

export function PerformanceChart(
  { executionResult, width = 400, height = 300 }: PerformanceChartProps,
) {
  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!svgRef.current || !executionResult) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove(); // Clear previous content

    const margin = { top: 20, right: 30, bottom: 40, left: 60 };
    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;

    // Data for the chart
    const data = [
      {
        language: "Python",
        executionTime: executionResult.python.execution_time_ms,
        color: "#F59E0B", // yellow-500
      },
      {
        language: "Rust",
        executionTime: executionResult.rust.execution_time_ms,
        color: "#10B981", // green-500
      },
    ];

    // Scales
    const xScale = d3.scaleBand()
      .domain(data.map((d) => d.language))
      .range([0, innerWidth])
      .padding(0.3);

    const yScale = d3.scaleLinear()
      .domain([0, d3.max(data, (d) => d.executionTime) || 100])
      .nice()
      .range([innerHeight, 0]);

    // Create main group
    const g = svg.append("g")
      .attr("transform", `translate(${margin.left}, ${margin.top})`);

    // Create bars with animation
    const bars = g.selectAll(".bar")
      .data(data)
      .enter()
      .append("g")
      .attr("class", "bar");

    // Add bar rectangles
    bars.append("rect")
      .attr("x", (d) => xScale(d.language) || 0)
      .attr("width", xScale.bandwidth())
      .attr("y", innerHeight) // Start from bottom
      .attr("height", 0) // Start with 0 height
      .attr("fill", (d) => d.color)
      .attr("rx", 4) // Rounded corners
      .transition()
      .duration(750)
      .ease(d3.easeCubicOut)
      .attr("y", (d) => yScale(d.executionTime))
      .attr("height", (d) => innerHeight - yScale(d.executionTime));

    // Add value labels on bars
    bars.append("text")
      .attr("x", (d) => (xScale(d.language) || 0) + xScale.bandwidth() / 2)
      .attr("y", innerHeight)
      .attr("text-anchor", "middle")
      .attr("fill", "white")
      .attr("font-weight", "bold")
      .attr("font-size", "12px")
      .attr("font-family", "Inter, system-ui, sans-serif")
      .text((d) => `${d.executionTime.toFixed(1)}ms`)
      .transition()
      .duration(750)
      .delay(300)
      .attr("y", (d) => yScale(d.executionTime) + 20);

    // Add speedup indicator
    const speedup = executionResult.python.execution_time_ms /
      executionResult.rust.execution_time_ms;
    if (speedup > 1) {
      g.append("text")
        .attr("x", innerWidth / 2)
        .attr("y", -5)
        .attr("text-anchor", "middle")
        .attr("fill", "#059669") // green-600
        .attr("font-weight", "bold")
        .attr("font-size", "14px")
        .attr("font-family", "Inter, system-ui, sans-serif")
        .text(`${speedup.toFixed(1)}× faster`)
        .style("opacity", 0)
        .transition()
        .duration(500)
        .delay(1000)
        .style("opacity", 1);
    }

    // X Axis
    g.append("g")
      .attr("transform", `translate(0, ${innerHeight})`)
      .call(d3.axisBottom(xScale))
      .selectAll("text")
      .attr("font-family", "Inter, system-ui, sans-serif")
      .attr("font-size", "12px");

    // Y Axis
    g.append("g")
      .call(d3.axisLeft(yScale).tickFormat((d) => `${d}ms`))
      .selectAll("text")
      .attr("font-family", "Inter, system-ui, sans-serif")
      .attr("font-size", "12px");

    // Y Axis label
    g.append("text")
      .attr("transform", "rotate(-90)")
      .attr("y", -40)
      .attr("x", -innerHeight / 2)
      .attr("text-anchor", "middle")
      .attr("fill", "#6B7280") // gray-500
      .attr("font-size", "12px")
      .attr("font-family", "Inter, system-ui, sans-serif")
      .text("Execution Time (ms)");

    // Add grid lines
    g.append("g")
      .attr("class", "grid")
      .call(
        d3.axisLeft(yScale)
          .tickSize(-innerWidth)
          .tickFormat(() => ""),
      )
      .selectAll("line")
      .attr("stroke", "#E5E7EB") // gray-200
      .attr("stroke-dasharray", "3,3");
  }, [executionResult, width, height]);

  if (!executionResult) {
    return (
      <div className="bg-white rounded-lg shadow-sm p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Performance Comparison</h3>
        <div className="flex items-center justify-center h-64 text-gray-500">
          <div className="text-center">
            <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
              <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                />
              </svg>
            </div>
            <p>Run code to see performance comparison</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <h3 className="text-lg font-semibold text-gray-900 mb-4">Performance Comparison</h3>
      <svg ref={svgRef} width={width} height={height} />

      <div className="mt-4 grid grid-cols-2 gap-4 text-sm">
        <div className="bg-yellow-50 rounded-lg p-3 border border-yellow-200">
          <div className="flex items-center space-x-2 mb-1">
            <div className="w-3 h-3 bg-yellow-500 rounded"></div>
            <span className="font-semibold text-yellow-800">Python</span>
          </div>
          <p className="text-yellow-700">
            {executionResult.python.execution_time_ms.toFixed(2)}ms execution time
          </p>
        </div>

        <div className="bg-green-50 rounded-lg p-3 border border-green-200">
          <div className="flex items-center space-x-2 mb-1">
            <div className="w-3 h-3 bg-green-500 rounded"></div>
            <span className="font-semibold text-green-800">Rust</span>
          </div>
          <p className="text-green-700">
            {executionResult.rust.execution_time_ms.toFixed(2)}ms execution time
            {executionResult.rust.compilation_time_ms && (
              <span className="block text-xs">
                (+{executionResult.rust.compilation_time_ms.toFixed(2)}ms compile)
              </span>
            )}
          </p>
        </div>
      </div>
    </div>
  );
}
