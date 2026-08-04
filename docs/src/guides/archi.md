# Import into Archi

`archr generate` produces an `.archimate` file (Archi native XML, namespace `http://www.archimatetool.com/archimate`, version `5.0.0`) with layout coordinates and a default diagram view.

## Steps

1. Generate the file:

   ```bash
   archr generate --input model.yaml --output model.archimate
   ```

2. Open [Archi](https://www.archimatetool.com).

3. **File → Import → Import Model from File…** and select `model.archimate`.

4. The model appears in the Models tree with a default view containing all elements positioned by `archr`'s topological grid layout.

## Round-trip

Edit in Archi, export, and bring back to YAML:

```bash
archr parse --input model.archimate --output model.yaml
```

Or compare two versions:

```bash
archr diff --old model-v1.archimate --new model-v2.yaml
```
