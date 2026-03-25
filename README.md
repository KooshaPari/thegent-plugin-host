# thegent-plugin-host

Plugin host and loader for thegent multi-agent system.

## Architecture

- **Hexagonal Architecture**: Ports & Adapters pattern
- **Clean Architecture**: Domain/Application/Ports/Adapters layers
- **DDD**: Entities, Value Objects, Domain Events

## Structure

```
src/
├── domain/           # Core business logic
│   ├── entities/    # Plugin, PluginId
│   ├── value_objects/ # PluginName, Priority
│   └── events/      # PluginEvent
├── application/      # Use cases
│   ├── commands/    # CQRS Commands
│   ├── queries/     # CQRS Queries
│   └── use_cases/   # PluginUseCase
├── ports/           # Interfaces
│   ├── driven/      # Storage, Loader, Event ports
│   └── driving/    # CLI, API ports
└── adapters/       # Implementations
    ├── inmemory/    # Testing adapter
    ├── dynamic/     # Dynamic library loader
    └── wasm/        # WASM loader
```

## Usage

```bash
# List plugins
thegent-plugin-host list

# Install plugin
thegent-plugin-host install my-plugin --version 1.0.0

# Uninstall plugin
thegent-plugin-host uninstall my-plugin

# Enable plugin
thegent-plugin-host enable my-plugin

# Disable plugin
thegent-plugin-host disable my-plugin
```

## License

MIT
