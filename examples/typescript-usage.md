# TypeScript Usage Examples

## Installation

### From Release (Recommended for sid-note)

Download the WASM package from the latest release:

```bash
# For web projects
curl -L https://github.com/kako-jun/sid-fret/releases/latest/download/sid-fret-wasm-web.tar.gz | tar xz -C ./wasm

# For Node.js projects
curl -L https://github.com/kako-jun/sid-fret/releases/latest/download/sid-fret-wasm-nodejs.tar.gz | tar xz -C ./wasm
```

### From Local Build

```bash
# Clone and build
git clone https://github.com/kako-jun/sid-fret.git
cd sid-fret
npm run build:wasm
# or
npm run build:wasm:nodejs
```

## Usage in TypeScript/JavaScript

### Web Project (Next.js, Vite, etc.)

```typescript
import init, {
  calculate_fingering,
  get_fret_offset,
  get_functional_harmony,
  cadence_text,
  is_chromatic_note,
  get_chord_name_aliases,
} from './wasm/sid_fret';

// Initialize WASM module
await init();

// 1. Fingering Calculation
const pitches = new Uint8Array([0, 3, 5, 7]); // E-G-A-B
const pattern = calculate_fingering(pitches, "shortest");

console.log('Positions:', pattern.positions);
console.log('Score:', pattern.score);
console.log('Algorithm:', pattern.algorithm);

// Access individual positions
pattern.positions.forEach((pos, i) => {
  console.log(`Note ${i}: String ${pos.string}, Fret ${pos.fret}`);
});

// 2. Different Fingering Modes
const shortest = calculate_fingering(pitches, "shortest");
const positionStable = calculate_fingering(pitches, "position-stable");
const openString = calculate_fingering(pitches, "open-string");
const stringPriority = calculate_fingering(pitches, "string-priority");
const balanced = calculate_fingering(pitches, "balanced");

// 3. Fret Offset Calculation
const offset = get_fret_offset("C");  // 0
const offsetG = get_fret_offset("G"); // 7

// 4. Functional Harmony Analysis
const degree = get_functional_harmony("C", "G"); // 5 (V)
const cadence = cadence_text(5, 1); // "Perfect Cadence" (V→I)

// 5. Japanese Notation Support
const isChromatic = is_chromatic_note("C2", "C＃2"); // true
const aliases = get_chord_name_aliases("Cmaj7");
// ["Cmaj7", "CM7", "C△7"]
```

### Node.js Project

```typescript
import { readFile } from 'fs/promises';
import { calculate_fingering } from './wasm-node/sid_fret';

// Initialize with WASM file
const wasmBuffer = await readFile('./wasm-node/sid_fret_bg.wasm');
const wasmModule = await WebAssembly.instantiate(wasmBuffer);

// Use functions
const pitches = new Uint8Array([0, 3, 5, 7]);
const pattern = calculate_fingering(pitches, "balanced");
console.log(pattern);
```

## Integration with sid-note (Next.js)

### 1. Add to your project

```bash
# In your sid-note project
mkdir -p public/wasm
cd public/wasm
curl -L https://github.com/kako-jun/sid-fret/releases/latest/download/sid-fret-wasm-web.tar.gz | tar xz
```

### 2. Create a hook

```typescript
// hooks/useSidFret.ts
import { useEffect, useState } from 'react';
import init, { calculate_fingering } from '@/public/wasm/sid_fret';

export function useSidFret() {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    init().then(() => setInitialized(true));
  }, []);

  return {
    initialized,
    calculateFingering: (pitches: number[], mode: string) => {
      if (!initialized) return null;
      return calculate_fingering(new Uint8Array(pitches), mode);
    },
  };
}
```

### 3. Use in components

```typescript
// components/FingeringDisplay.tsx
'use client';

import { useSidFret } from '@/hooks/useSidFret';

export default function FingeringDisplay({ notes }: { notes: number[] }) {
  const { initialized, calculateFingering } = useSidFret();

  if (!initialized) return <div>Loading WASM...</div>;

  const pattern = calculateFingering(notes, 'balanced');
  if (!pattern) return null;

  return (
    <div>
      <h2>Fingering (Score: {pattern.score})</h2>
      <ul>
        {pattern.positions.map((pos: any, i: number) => (
          <li key={i}>
            String {pos.string}, Fret {pos.fret}
            {pos.finger && ` (Finger ${pos.finger})`}
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### 4. Mode selector component

```typescript
// components/FingeringModeSelector.tsx
'use client';

import { useState } from 'react';
import { useSidFret } from '@/hooks/useSidFret';

const MODES = [
  { value: 'shortest', label: '最短移動' },
  { value: 'position-stable', label: 'ポジション固定' },
  { value: 'string-priority', label: '弦移動優先' },
  { value: 'open-string', label: '開放弦活用' },
  { value: 'balanced', label: 'バランス型' },
];

export default function FingeringModeSelector({
  notes
}: {
  notes: number[]
}) {
  const [mode, setMode] = useState('balanced');
  const { initialized, calculateFingering } = useSidFret();

  if (!initialized) return null;

  const pattern = calculateFingering(notes, mode);

  return (
    <div>
      <select value={mode} onChange={(e) => setMode(e.target.value)}>
        {MODES.map(({ value, label }) => (
          <option key={value} value={value}>
            {label}
          </option>
        ))}
      </select>

      {pattern && (
        <div>
          <p>Score: {pattern.score.toFixed(2)}</p>
          <p>Algorithm: {pattern.algorithm}</p>
          {/* Render fingering positions */}
        </div>
      )}
    </div>
  );
}
```

## Type Definitions

WASM-pack automatically generates TypeScript definitions. You can find them in:
- `pkg/sid_fret.d.ts` (web target)
- `pkg-node/sid_fret.d.ts` (nodejs target)

Example type definitions:

```typescript
export function calculate_fingering(
  pitches: Uint8Array,
  mode: string
): FingeringPattern;

export interface FingeringPattern {
  positions: FretPosition[];
  score: number;
  algorithm: string;
}

export interface FretPosition {
  string: number;
  fret: number;
  finger?: number;
}

export function get_fret_offset(root: string): number;
export function get_functional_harmony(scale: string, chord: string): number;
export function cadence_text(prev: number, curr: number): string;
export function is_chromatic_note(
  note_pitch: string | null,
  next_note_pitch: string | null
): boolean;
export function get_chord_name_aliases(chord_name: string): string[];
```

## Performance Tips

### 1. Initialize once
```typescript
// Don't initialize in every component
let wasmInitialized = false;

export async function initWasm() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}
```

### 2. Cache results
```typescript
const cache = new Map<string, FingeringPattern>();

function getCachedFingering(notes: number[], mode: string) {
  const key = `${notes.join(',')}-${mode}`;
  if (!cache.has(key)) {
    cache.set(key, calculateFingering(notes, mode));
  }
  return cache.get(key);
}
```

### 3. Web Worker for heavy calculations
```typescript
// fingering.worker.ts
import init, { calculate_fingering } from './wasm/sid_fret';

let initialized = false;

self.onmessage = async (e) => {
  if (!initialized) {
    await init();
    initialized = true;
  }

  const { pitches, mode } = e.data;
  const result = calculate_fingering(new Uint8Array(pitches), mode);
  self.postMessage(result);
};
```

## Troubleshooting

### WASM Loading Error
Make sure the WASM file is accessible:
```typescript
// Next.js: Place in public/ directory
import init from '@/public/wasm/sid_fret';

// Vite: Place in public/ directory
import init from '/wasm/sid_fret';
```

### TypeScript Type Errors
```bash
# Generate types
npm run build:wasm
# Types are in pkg/sid_fret.d.ts
```

### Memory Issues
```typescript
// Clean up when done
const pattern = calculateFingering(notes, mode);
// Use pattern...
// WASM memory is automatically managed by Rust
```
