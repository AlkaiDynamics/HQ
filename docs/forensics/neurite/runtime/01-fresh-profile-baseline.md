# Neurite Runtime Observation — Fresh Profile Baseline

## Specimen

- Desktop release: `electron-2025.06.18.233853`
- Windows installer SHA-256: `faa2506f579ec2bef1d790f39670c115ad1acb61588e207d7e9d0290a9b8d35c`
- Observation class: `RUNTIME-OBSERVED`
- Scenario: first launch / fresh profile, before sign-in, API-key entry, workspace import, or settings changes

## Observed startup behavior

The application successfully launched into a native desktop window titled `Neurite`.

Observed visible startup surface:

- black spatial/fractal canvas fills essentially the entire client area;
- Mandelbrot/fractal contours are visible as orange/yellow/white/green/blue curved segments on a dark field;
- darker internal filament/branch geometry is visible through the center of the scene;
- a hamburger/menu control is visible at upper right;
- no sign-in dialog, onboarding modal, API-key prompt, workspace picker, or fatal error dialog is visible;
- startup reached an interactive-looking canvas rather than failing on a missing cloud/service dependency;
- the same startup surface was captured in both large and smaller window geometries.

This establishes only the visible fresh-start baseline. It does **not** yet establish successful interaction, persistence, offline behavior, missing-service behavior, local/cloud AI behavior, or failure handling.

## Capture artifacts

Conversation capture 1:

- dimensions: `1919x1079`
- SHA-256: `236ea79278c3000f2a706f2b13c3ace0f29e0502f655a2d826e07f80090b282e`

Conversation capture 2:

- dimensions: `1319x879`
- SHA-256: `91df714b133408a7444c48374ad22f526b272cb8706b131032cb4929857e859c`

Conversation capture 3:

- dimensions: `1919x1079`
- SHA-256: `f1c3fbd36931f7191f59dcef2517023e08008823afcad02a840016634b0d12f0`

The binary screenshots currently remain conversation artifacts; these hashes identify the exact captures used for this observation.

## Evidence status

`Fresh-profile runtime reproduction` → **PARTIALLY VERIFIED**

Verified:

- exact desktop specimen launches;
- main window renders;
- primary fractal/spatial startup surface appears;
- no immediate hard dependency prompt/failure is visible.

Still required before this scenario is closed:

- prove first interaction works;
- capture initial state before/after mutation;
- close/reopen behavior;
- determine whether any hidden/local service startup failures occurred;
- capture relevant process/network/storage effects where practical.

## Next runtime fixture

Begin the core interaction sequence with creation of a single note/text node, then capture its visible result before performing any other mutation.
