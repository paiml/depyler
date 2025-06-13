import { useEffect, useMemo, useRef } from "react";
import * as d3 from "d3";
import { EnergyEstimate } from "@/types";

interface EnergyGaugeProps {
  savings: number;
  energyData: {
    python: EnergyEstimate;
    rust: EnergyEstimate;
  };
  confidence: number;
}

export function EnergyGauge({ savings, energyData, confidence }: EnergyGaugeProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Memoize scales to prevent recreation
  const scales = useMemo(() => ({
    savings: d3.scaleLinear().domain([0, 100]).range([-Math.PI / 2, Math.PI / 2]),
    color: d3.scaleSequential()
      .domain([0, 100])
      .interpolator(d3.interpolateRdYlGn),
  }), []);

  useEffect(() => {
    if (!svgRef.current || !containerRef.current) return;

    const svg = d3.select(svgRef.current);
    const { width, height } = containerRef.current.getBoundingClientRect();
    const radius = Math.min(width, height) / 2 - 20;

    // Clear previous content
    svg.selectAll("*").remove();

    // Use D3's idiomatic enter/update/exit pattern
    const g = svg.selectAll<SVGGElement, number>("g.gauge-container")
      .data([savings], (d) => d)
      .join(
        (enter) =>
          enter.append("g")
            .attr("class", "gauge-container")
            .call((g) => g.append("path").attr("class", "gauge-background"))
            .call((g) => g.append("path").attr("class", "gauge-value"))
            .call((g) => g.append("text").attr("class", "gauge-text"))
            .call((g) => g.append("text").attr("class", "gauge-label"))
            .call((g) => g.append("text").attr("class", "gauge-confidence")),
        (update) => update,
        (exit) => exit.remove(),
      )
      .attr("transform", `translate(${width / 2}, ${height - 20})`);

    // Background arc
    const backgroundArc = d3.arc<any>()
      .innerRadius(radius * 0.7)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2)
      .endAngle(Math.PI / 2);

    g.select(".gauge-background")
      .attr("d", backgroundArc)
      .style("fill", "#e0e0e0")
      .style("stroke", "#ccc")
      .style("stroke-width", 1);

    // Value arc with smooth transition
    const valueArc = d3.arc<any>()
      .innerRadius(radius * 0.7)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2);

    const valueArcPath = g.select(".gauge-value");

    valueArcPath
      .datum({ endAngle: scales.savings(savings) })
      .style("fill", scales.color(savings))
      .style("stroke", "#fff")
      .style("stroke-width", 2)
      .transition()
      .duration(750)
      .ease(d3.easeCubicInOut)
      .attrTween("d", function (d) {
        const node = this as any;
        const interpolate = d3.interpolate(
          node._current || { endAngle: -Math.PI / 2 },
          d,
        );
        node._current = interpolate(1);
        return (t: number) => valueArc(interpolate(t)) || "";
      });

    // Animated text
    const gaugeText = g.select(".gauge-text");
    gaugeText
      .attr("text-anchor", "middle")
      .attr("dy", "-0.5em")
      .style("font-size", "2.5em")
      .style("font-weight", "bold")
      .style("font-family", "Inter, system-ui, sans-serif")
      .style("fill", scales.color(savings))
      .transition()
      .duration(750)
      .tween("text", function () {
        const node = this as any;
        const interpolate = d3.interpolate(
          node._current || 0,
          savings,
        );
        node._current = savings;
        return function (t: number) {
          (this as any).textContent = `${Math.round(interpolate(t))}%`;
        };
      });

    // Confidence indicator
    g.select(".gauge-confidence")
      .attr("text-anchor", "middle")
      .attr("dy", "1em")
      .style("font-size", "0.9em")
      .style("font-family", "Inter, system-ui, sans-serif")
      .style("fill", "#666")
      .text(`Confidence: ${Math.round(confidence * 100)}%`);

    // Energy savings label
    g.select(".gauge-label")
      .attr("text-anchor", "middle")
      .attr("dy", "2.5em")
      .style("font-size", "1.1em")
      .style("font-weight", "600")
      .style("font-family", "Inter, system-ui, sans-serif")
      .style("fill", "#374151")
      .text("Energy Savings");
  }, [savings, confidence, scales]);

  return (
    <div ref={containerRef} className="energy-gauge-container bg-white rounded-lg shadow-sm p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">Energy Efficiency</h3>
        <div className="flex items-center space-x-2">
          <div className="w-3 h-3 bg-green-500 rounded-full"></div>
          <span className="text-sm text-gray-600">Rust</span>
          <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
          <span className="text-sm text-gray-600">Python</span>
        </div>
      </div>

      <svg ref={svgRef} width="100%" height="200" />

      <EnergyBreakdownDetails energyData={energyData} />
    </div>
  );
}

interface EnergyBreakdownDetailsProps {
  energyData: {
    python: EnergyEstimate;
    rust: EnergyEstimate;
  };
}

function EnergyBreakdownDetails({ energyData }: EnergyBreakdownDetailsProps) {
  const formatEnergy = (joules: number) => {
    if (joules < 0.001) return `${(joules * 1000000).toFixed(2)} μJ`;
    if (joules < 1) return `${(joules * 1000).toFixed(2)} mJ`;
    return `${joules.toFixed(3)} J`;
  };

  const formatCO2 = (grams: number) => {
    if (grams < 0.001) return `${(grams * 1000000).toFixed(2)} μg`;
    if (grams < 1) return `${(grams * 1000).toFixed(2)} mg`;
    return `${grams.toFixed(3)} g`;
  };

  return (
    <div className="mt-6 space-y-4">
      <div className="grid grid-cols-2 gap-4">
        <div className="bg-yellow-50 rounded-lg p-4 border border-yellow-200">
          <h4 className="text-sm font-semibold text-yellow-800 mb-2">Python</h4>
          <div className="space-y-1 text-sm">
            <div className="flex justify-between">
              <span className="text-yellow-700">Energy:</span>
              <span className="font-mono text-yellow-900">
                {formatEnergy(energyData.python.joules)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-yellow-700">CO₂:</span>
              <span className="font-mono text-yellow-900">
                {formatCO2(energyData.python.co2Grams)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-yellow-700">CPU:</span>
              <span className="font-mono text-yellow-900">
                {formatEnergy(energyData.python.breakdown.cpu)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-yellow-700">Memory:</span>
              <span className="font-mono text-yellow-900">
                {formatEnergy(energyData.python.breakdown.memory)}
              </span>
            </div>
          </div>
        </div>

        <div className="bg-green-50 rounded-lg p-4 border border-green-200">
          <h4 className="text-sm font-semibold text-green-800 mb-2">Rust</h4>
          <div className="space-y-1 text-sm">
            <div className="flex justify-between">
              <span className="text-green-700">Energy:</span>
              <span className="font-mono text-green-900">
                {formatEnergy(energyData.rust.joules)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-green-700">CO₂:</span>
              <span className="font-mono text-green-900">
                {formatCO2(energyData.rust.co2Grams)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-green-700">CPU:</span>
              <span className="font-mono text-green-900">
                {formatEnergy(energyData.rust.breakdown.cpu)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-green-700">Memory:</span>
              <span className="font-mono text-green-900">
                {formatEnergy(energyData.rust.breakdown.memory)}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-blue-50 rounded-lg p-4 border border-blue-200">
        <h4 className="text-sm font-semibold text-blue-800 mb-2">Environmental Impact</h4>
        <div className="text-sm text-blue-700">
          <p className="mb-1">
            <strong>Energy equivalent:</strong> {energyData.rust.equivalentTo}
          </p>
          <p>
            <strong>CO₂ reduction:</strong>{" "}
            {formatCO2(energyData.python.co2Grams - energyData.rust.co2Grams)} per execution
          </p>
        </div>
      </div>
    </div>
  );
}
