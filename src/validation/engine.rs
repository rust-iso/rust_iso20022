/// Non-owning validation context. Profile releases are explicit rather than
/// implied by a profile name.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidationContext<'a> {
    pub message_id: Option<&'a str>,
    pub profile: Option<&'a str>,
    pub profile_version: Option<&'a str>,
}

/// A short-lived field view used by reusable rules. It cannot become a second
/// canonical message model and is intended to be projected from generated
/// structs or streaming XML events.
#[derive(Debug, Clone, Default)]
pub struct ValidationTarget<'a> {
    fields: Vec<(&'a str, &'a str)>,
}

/// Availability of one layer in a concrete validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerAvailability {
    Available,
    Unavailable(&'static str),
}

/// Deterministic L1 → L2 → L3 executor over one shared rule registry.
#[derive(Debug)]
pub struct ValidationPipeline<'a> {
    registry: super::RuleRegistry<'a>,
    layers: [LayerAvailability; 3],
}

impl<'a> ValidationPipeline<'a> {
    pub fn new(registry: super::RuleRegistry<'a>) -> Self {
        Self {
            registry,
            layers: [
                LayerAvailability::Unavailable("syntax/schema layer not configured"),
                LayerAvailability::Unavailable("ISO semantic layer not configured"),
                LayerAvailability::Unavailable("profile layer not configured"),
            ],
        }
    }

    pub fn with_layer(
        mut self,
        layer: super::ValidationLayer,
        availability: LayerAvailability,
    ) -> Self {
        self.layers[layer_index(layer)] = availability;
        self
    }

    pub fn validate(
        &self,
        target: &ValidationTarget<'_>,
        context: &ValidationContext<'_>,
    ) -> super::ValidationReport {
        let mut report = self.registry.validate_layers(target, context, |layer| {
            self.layers[layer_index(layer)] == LayerAvailability::Available
        });
        for layer in [
            super::ValidationLayer::SyntaxSchema,
            super::ValidationLayer::IsoSemantic,
            super::ValidationLayer::Profile,
        ] {
            if let LayerAvailability::Unavailable(reason) = self.layers[layer_index(layer)] {
                report.mark_unavailable(layer, reason);
            }
        }
        report.sort();
        report
    }
}

const fn layer_index(layer: super::ValidationLayer) -> usize {
    match layer {
        super::ValidationLayer::SyntaxSchema => 0,
        super::ValidationLayer::IsoSemantic => 1,
        super::ValidationLayer::Profile => 2,
    }
}

impl<'a> ValidationTarget<'a> {
    pub fn from_pairs(fields: &[(&'a str, &'a str)]) -> Self {
        Self {
            fields: fields.to_vec(),
        }
    }

    pub fn text(&self, path: &str) -> Option<&'a str> {
        self.fields
            .iter()
            .find_map(|(candidate, value)| (*candidate == path).then_some(*value))
    }

    /// All non-empty values projected at a logical generated-field path.
    pub fn values<'b>(&'b self, path: &'b str) -> impl Iterator<Item = &'a str> + 'b {
        self.fields.iter().filter_map(move |(candidate, value)| {
            (*candidate == path && !value.is_empty()).then_some(*value)
        })
    }

    pub fn present(&self, path: &str) -> bool {
        self.values(path).next().is_some()
    }

    pub fn count(&self, path: &str) -> usize {
        self.values(path).count()
    }
}
