# AGENTS.md - Developer Guidelines for PersonalFinanceApp

## Overview

PersonalFinanceApp is a Tauri + SvelteKit + TypeScript desktop application for personal finance management. The frontend uses Svelte 5 with runes, TailwindCSS 4, and Skeleton UI components.

## Build Commands

### Frontend (Node.js)

| Command | Description |
|---------|-------------|
| `npm run dev` | Start Vite development server |
| `npm run build` | Build for production |
| `npm run preview` | Preview production build |
| `npm run check` | Run Svelte type checking (svelte-check) |
| `npm run check:watch` | Watch mode for type checking |

### Tauri (Desktop App)

| Command | Description |
|---------|-------------|
| `npm run tauri dev` | Run Tauri in development mode |
| `npm run tauri:linux` | Run Tauri in development mode (Linux, WebKit compositor fix) |
| `npm run tauri build` | Build production desktop app |
| `npm run tauri build -- --debug` | Build debug desktop app |

### Running a Single Test

This project does not currently have a test framework configured. If tests are added:
- Use `vitest` for unit tests
- Run single test: `npx vitest run --testNamePattern="test name"`

## Code Style Guidelines

### TypeScript

- **Strict mode enabled** in `tsconfig.json` - all strict checks are on
- Use explicit types for function parameters and return types
- Use `type` for object shapes, `enum` for fixed constants
- Avoid `any` - use `unknown` if type is truly unknown

### Svelte 5 (Runes)

This project uses Svelte 5 with runes:

```svelte
<script lang="ts">
  // State using $state
  let count = $state(0);
  
  // Derived values using $derived
  let doubled = $derived(count * 2);
  
  // Effects using $effect
  $effect(() => {
    console.log(count);
  });
  
  // Props using $props
  let { title, onClick }: { title: string; onClick: () => void } = $props();
</script>
```

- Use `$state` for reactive state
- Use `$derived` for computed values
- Use `$effect` for side effects (not onMount for everything)
- Use `$props` instead of `export let`

### Import Conventions

- **$lib/** - Import from `src/lib` for shared utilities, state, and components
- **Relative imports** - Use relative paths for domain/DTO imports
- **$app/** - SvelteKit built-in imports (navigation, stores)

```typescript
// Good
import { workspace } from "$lib/state/workspace.svelte";
import { handleCommandError } from "$lib/utils/error.handler";
import type { AccountInfoDto } from "../../domain/dto/account_info.dto";

// Avoid
import { workspace } from "/src/lib/state/workspace.svelte";
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Variables, functions | camelCase | `getRecords()`, `lastRecords` |
| Types, interfaces, enums | PascalCase | `AccountInfoDto`, `AppErrors` |
| Components | PascalCase | `SummaryCards.svelte` |
| Files (utils, handlers) | camelCase/snake_case | `error.handler.ts`, `formatter.svelte.ts` |
| DTOs | PascalCase with suffix | `AccountInfoDto`, `PaginationDto` |

### File Organization

```
src/
├── lib/
│   ├── state/          # Svelte state (runes-based)
│   ├── utils/         # Utility functions and handlers
│   └── toaster.ts     # Toast notifications
├── domain/
│   ├── dto/           # Data Transfer Objects
│   └── enums/         # TypeScript enums
├── routes/            # SvelteKit pages
│   └── app/
│       └── home/
│           ├── +page.svelte
│           └── components/
└── app.css            # Global styles
```

### Error Handling

Use the centralized error handling pattern:

```typescript
import { AppErrors } from "../../domain/enums/errors.enum";
import { handleCommandError } from "$lib/utils/error.handler";

// In async functions
try {
  const result = await invoke("some_command");
} catch (error: any) {
  handleCommandError(error);
}
```

Define errors in `src/domain/enums/errors.enum.ts`:
```typescript
export enum AppErrors {
  WorkspaceExists = "WorkspaceExists",
  IoError = "IoError",
  ConfigError = "ConfigError",
  DatabaseError = "DatabaseError",
  NotFound = "NotFound",
}
```

### Styling

- **TailwindCSS 4** - Use utility classes
- **@reference "tailwindcss"** - In Svelte `<style>` blocks
- **Skeleton UI** - Use Skeleton components for consistent design
- Use `preset-` classes from Skeleton for theming

```svelte
<style lang="postcss">
  @reference "tailwindcss";
</style>
```

### Component Patterns

- Use `<script lang="ts">` for TypeScript
- Prefer functional components with runes
- Use `onclick` instead of `on:click` (Svelte 5)
- Use `{@render children()}` for slot content

```svelte
<script lang="ts">
  let { children, title }: { children: any; title: string } = $props();
</script>

<div>
  <h1>{title}</h1>
  {@render children()}
</div>
```

### Tauri Integration

- Use `@tauri-apps/api/core` `invoke()` for backend commands
- Define Tauri commands in Rust (`src-tauri/src/`)
- Use plugins for filesystem (`@tauri-apps/plugin-fs`), dialogs (`@tauri-apps/plugin-dialog`)

## Recommended IDE Setup

- VS Code
- Svelte extension
- Tauri extension
- rust-analyzer (for Rust backend)
- TailwindCSS IntelliSense

## Type Checking

Always run `npm run check` before committing to ensure type safety. Fix all TypeScript and Svelte errors.
