# IL2Resolver

A tool that generates UnityResolver api calls using IL.

## Usage

```bash
IL2Resolver.exe --help

IL2Resolver.exe -d "path/to/assembly.dll" -o "./output"

IL2Resolver.exe -d "path/to/assembly.dll" -n "UnityEngine" -o "./output"

IL2Resolver.exe -d "path/to/assembly.dll" -t "Camera" -t "Transform" -o "./output"
```

## Build

```sh
dotnet build
```

## Options

- `-d, --dll`: Path to the .NET DLL to analyze (Required)
- `-o, --output`: Output directory for generated files (Default: ./output)
- `-r, --reference`: Optional reference .cs files for attribute hints
- `-n, --namespace`: Filter types by namespace
- `-t, --type`: Filter specific types
- `-v, --verbose`: Enable verbose debug logging
- `-sw, --suppress-warnings`: Suppress warning messages
