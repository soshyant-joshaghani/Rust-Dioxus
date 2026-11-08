# Runtime profiles

| Profile | Command | Redis | Worker |
|---------|---------|-------|--------|
| Full | `backend run dev` | yes | yes |
| Slim | `backend run dev --slim` | no | no |

Slim is a supported lightweight mode for CRUD and auth work. Production is always full.
