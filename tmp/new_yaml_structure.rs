// Estrutura nova do YAML com múltiplos viewpoints

/// Internal DTO for YAML unmarshaling.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlModel {
    model: YamlModelInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlModelInner {
    name: String,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
    #[serde(default)]
    viewpoints: Vec<YamlViewpointDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum YamlViewpointKind {
    None,
    Business,
    Application,
    Implementation,
    Motivation,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlViewpointDefinition {
    id: String,
    name: String,
    #[serde(rename_all = "lowercase")]
    kind: YamlViewpointKind,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlElement {
    id: String,
    name: String,
    kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlRelationship {
    id: String,
    source: String,
    target: String,
    kind: String,
}

// ... resto do código (functions de parsing e serialization)
