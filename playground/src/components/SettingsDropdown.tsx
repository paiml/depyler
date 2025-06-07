import React, { useState, useRef, useEffect } from "react";

interface Setting {
  id: string;
  label: string;
  type: "toggle" | "select";
  value: boolean | string;
  options?: { value: string; label: string }[];
  description?: string;
}

const defaultSettings: Setting[] = [
  {
    id: "optimize_energy",
    label: "Optimize for Energy",
    type: "toggle",
    value: true,
    description: "Enable energy-efficient code generation",
  },
  {
    id: "safety_level",
    label: "Safety Level",
    type: "select",
    value: "strict",
    options: [
      { value: "strict", label: "Strict" },
      { value: "moderate", label: "Moderate" },
      { value: "unsafe", label: "Unsafe" },
    ],
    description: "Control the level of safety checks in generated code",
  },
  {
    id: "string_strategy",
    label: "String Strategy",
    type: "select",
    value: "zero_copy",
    options: [
      { value: "zero_copy", label: "Zero Copy" },
      { value: "owned", label: "Owned" },
      { value: "cow", label: "Copy on Write" },
    ],
    description: "How strings are handled in the generated Rust code",
  },
  {
    id: "emit_docs",
    label: "Generate Documentation",
    type: "toggle",
    value: false,
    description: "Include documentation comments in generated code",
  },
  {
    id: "verify",
    label: "Verify Output",
    type: "toggle",
    value: true,
    description: "Run verification checks on generated code",
  },
];

export function SettingsDropdown() {
  const [isOpen, setIsOpen] = useState(false);
  const [settings, setSettings] = useState(defaultSettings);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleSettingChange = (id: string, value: boolean | string) => {
    setSettings(settings.map(s => s.id === id ? { ...s, value } : s));
  };

  return (
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        aria-expanded={isOpen}
        aria-haspopup="true"
      >
        <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
        Settings
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-80 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 z-50">
          <div className="py-1" role="menu" aria-orientation="vertical">
            <div className="px-4 py-3 border-b">
              <h3 className="text-lg font-medium text-gray-900">Transpiler Settings</h3>
              <p className="mt-1 text-sm text-gray-600">Configure transpilation options</p>
            </div>

            <div className="py-2 max-h-96 overflow-y-auto">
              {settings.map((setting) => (
                <div key={setting.id} className="px-4 py-3 hover:bg-gray-50">
                  <div className="flex items-start justify-between">
                    <div className="flex-1 mr-3">
                      <label
                        htmlFor={setting.id}
                        className="text-sm font-medium text-gray-900"
                      >
                        {setting.label}
                      </label>
                      {setting.description && (
                        <p className="text-xs text-gray-500 mt-1">{setting.description}</p>
                      )}
                    </div>
                    
                    {setting.type === "toggle" ? (
                      <button
                        id={setting.id}
                        role="switch"
                        aria-checked={setting.value as boolean}
                        onClick={() => handleSettingChange(setting.id, !(setting.value as boolean))}
                        className={`
                          relative inline-flex h-6 w-11 items-center rounded-full transition-colors
                          ${setting.value ? 'bg-blue-600' : 'bg-gray-200'}
                        `}
                      >
                        <span
                          className={`
                            inline-block h-4 w-4 transform rounded-full bg-white transition-transform
                            ${setting.value ? 'translate-x-6' : 'translate-x-1'}
                          `}
                        />
                      </button>
                    ) : (
                      <select
                        id={setting.id}
                        value={setting.value as string}
                        onChange={(e) => handleSettingChange(setting.id, e.target.value)}
                        className="text-sm border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                      >
                        {setting.options?.map(option => (
                          <option key={option.value} value={option.value}>
                            {option.label}
                          </option>
                        ))}
                      </select>
                    )}
                  </div>
                </div>
              ))}
            </div>

            <div className="px-4 py-3 border-t bg-gray-50">
              <button
                onClick={() => setSettings(defaultSettings)}
                className="text-sm text-gray-600 hover:text-gray-900"
              >
                Reset to defaults
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}