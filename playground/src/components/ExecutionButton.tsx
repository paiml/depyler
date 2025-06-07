import React, { useCallback, useState, useRef } from "react";
import { usePlaygroundStore } from "@/store";
import { LoadingState } from "@/types";

export function ExecutionButton() {
  const { executeCode, transpileCode, pythonCode, rustCode, isExecuting, isTranspiling } =
    usePlaygroundStore();
  const [loadingState, setLoadingState] = useState<LoadingState>({ type: "idle" });
  const isProcessingRef = useRef(false);

  const handleExecute = useCallback(async () => {
    if (!pythonCode.trim() || isProcessingRef.current || loadingState.type !== "idle" || isExecuting || isTranspiling) {
      return;
    }

    isProcessingRef.current = true;
    try {
      // First ensure we have transpiled code
      if (!rustCode.trim()) {
        setLoadingState({
          type: "compiling",
          message: "Transpiling Python to Rust...",
        });

        await transpileCode();
        await new Promise((resolve) => setTimeout(resolve, 100)); // UI update
      }

      setLoadingState({
        type: "executing",
        message: "Running performance comparison...",
      });

      await executeCode();
    } finally {
      setLoadingState({ type: "idle" });
      isProcessingRef.current = false;
    }
  }, [pythonCode, rustCode, transpileCode, executeCode, loadingState.type, isExecuting, isTranspiling]);

  const isLoading = loadingState.type !== "idle" || isExecuting || isTranspiling;
  const isDisabled = !pythonCode.trim() || isLoading;

  const renderButtonContent = () => {
    if (isTranspiling) {
      return (
        <div className="flex items-center space-x-2">
          <Spinner size={20} />
          <span>Transpiling...</span>
        </div>
      );
    }

    switch (loadingState.type) {
      case "downloading":
        return (
          <div className="flex items-center space-x-2">
            <CircularProgress value={loadingState.progress} size={20} />
            <div className="flex flex-col items-start">
              <span className="text-sm font-medium">Downloading Toolchain</span>
              <span className="text-xs text-gray-600">{loadingState.message}</span>
            </div>
          </div>
        );
      case "compiling":
      case "executing":
        return (
          <div className="flex items-center space-x-2">
            <Spinner size={20} />
            <span>{loadingState.message}</span>
          </div>
        );
      default:
        return (
          <div className="flex items-center space-x-2">
            <PlayIcon size={20} />
            <span>Run Comparison</span>
          </div>
        );
    }
  };

  return (
    <button
      type="button"
      onClick={handleExecute}
      disabled={isDisabled}
      className={`
        inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white
        ${
        isDisabled
          ? "bg-gray-400 cursor-not-allowed"
          : "bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      }
        transition-colors duration-200
      `}
      aria-busy={isLoading}
      aria-describedby={isLoading ? "execution-status" : undefined}
    >
      {renderButtonContent()}
      {isLoading && (
        <span id="execution-status" className="sr-only">
          {loadingState.message || "Processing..."}
        </span>
      )}
    </button>
  );
}

function Spinner({ size = 24 }: { size?: number }) {
  return (
    <svg
      className="animate-spin"
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth="4"
        className="opacity-25"
      />
      <path
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        className="opacity-75"
      />
    </svg>
  );
}

function CircularProgress({ value = 0, size = 24 }: { value?: number; size?: number }) {
  const radius = (size - 4) / 2;
  const circumference = radius * 2 * Math.PI;
  const strokeDasharray = `${circumference} ${circumference}`;
  const strokeDashoffset = circumference - (value / 100) * circumference;

  return (
    <div className="relative" style={{ width: size, height: size }} role="progressbar" aria-valuenow={Math.round(value)} aria-valuemin={0} aria-valuemax={100}>
      <svg
        className="transform -rotate-90"
        width={size}
        height={size}
        viewBox={`0 0 ${size} ${size}`}
      >
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          stroke="currentColor"
          strokeWidth="2"
          fill="transparent"
          className="opacity-25"
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          stroke="currentColor"
          strokeWidth="2"
          fill="transparent"
          strokeDasharray={strokeDasharray}
          strokeDashoffset={strokeDashoffset}
          className="opacity-75"
          style={{ transition: "stroke-dashoffset 0.3s ease" }}
        />
      </svg>
      <div className="absolute inset-0 flex items-center justify-center">
        <span className="text-xs font-medium">{Math.round(value)}%</span>
      </div>
    </div>
  );
}

function PlayIcon({ size = 24 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="currentColor"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M8 5v14l11-7z" />
    </svg>
  );
}
