//! Local presentation draft; these descriptors do not define executable flow semantics.
use conctr_ui_dioxus::ui::workflow::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    Trigger,
    Verify,
    Mapping,
    Condition,
    Call,
    Respond,
    Error,
    Delay,
}
impl StepKind {
    pub const ALL: [Self; 8] = [
        Self::Trigger,
        Self::Verify,
        Self::Mapping,
        Self::Condition,
        Self::Call,
        Self::Respond,
        Self::Error,
        Self::Delay,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Trigger => "HTTP trigger",
            Self::Verify => "Verify signature",
            Self::Mapping => "Mapping",
            Self::Condition => "Condition",
            Self::Call => "API call",
            Self::Respond => "Respond",
            Self::Error => "Error handler",
            Self::Delay => "Delay",
        }
    }
    pub fn group(self) -> &'static str {
        match self {
            Self::Trigger => "Triggers",
            Self::Verify | Self::Condition | Self::Delay => "Logic",
            Self::Mapping | Self::Call => "Data",
            Self::Respond | Self::Error => "Output",
        }
    }
    pub fn node(self, id: String, x: f64, y: f64) -> WorkflowNode {
        WorkflowNode {
            id,
            initial_x: x,
            initial_y: y,
            width: 208.0,
            has_target: self != Self::Trigger,
            has_source: !matches!(self, Self::Respond | Self::Error),
            label: self.label().into(),
            description: match self {
                Self::Trigger => "POST /hooks/orders",
                Self::Verify => "HMAC SHA-256",
                Self::Mapping => "Order to sales order",
                Self::Condition => "$.total_price > 1000",
                Self::Call => "POST /SalesOrder",
                Self::Respond => "202 · Order accepted",
                Self::Error => "Stop and require attention",
                Self::Delay => "1000 ms",
            }
            .into(),
            kind: match self {
                Self::Trigger => WorkflowNodeKind::Trigger,
                Self::Respond | Self::Error => WorkflowNodeKind::Output,
                _ => WorkflowNodeKind::Data,
            },
        }
    }
}
pub fn sample() -> (Vec<WorkflowNode>, Vec<WorkflowEdge>) {
    let steps = [
        (StepKind::Trigger, "Order created", 40., 160.),
        (StepKind::Verify, "Verify signature", 300., 160.),
        (StepKind::Mapping, "Map request", 560., 160.),
        (StepKind::Condition, "Amount over 1000?", 820., 160.),
        (StepKind::Call, "Request approval", 1080., 40.),
        (StepKind::Call, "Create sales order", 1080., 280.),
        (StepKind::Respond, "Respond 202", 1340., 160.),
        (StepKind::Error, "On error", 1340., 400.),
    ];
    let nodes = steps
        .into_iter()
        .enumerate()
        .map(|(i, (kind, title, x, y))| {
            let mut n = kind.node(format!("step-{i}"), x, y);
            n.label = title.into();
            if i == 4 {
                n.description = "POST /approvals".into();
            }
            n
        })
        .collect();
    let edges = [
        (0, 1, None),
        (1, 2, None),
        (2, 3, None),
        (3, 4, Some("Yes")),
        (3, 5, Some("No")),
        (4, 6, None),
        (5, 6, None),
        (5, 7, Some("On error")),
    ]
    .into_iter()
    .map(|(a, b, label)| WorkflowEdge {
        from: format!("step-{a}"),
        to: format!("step-{b}"),
        label: label.map(str::to_owned),
        ..Default::default()
    })
    .collect();
    (nodes, edges)
}
/// Presentation diagnostics only; never a deployment or runtime validation result.
pub fn draft_issues(nodes: &[WorkflowNode], edges: &[WorkflowEdge]) -> Vec<String> {
    let mut issues = Vec::new();
    if nodes.is_empty() {
        issues.push("Add a trigger and connect your first step.".into());
        return issues;
    }
    if nodes
        .iter()
        .filter(|n| n.kind == WorkflowNodeKind::Trigger)
        .count()
        != 1
    {
        issues.push("Use one entry trigger in this draft.".into());
    }
    for n in nodes {
        if n.label.trim().is_empty() {
            issues.push(format!("{} needs a step name.", n.id));
        }
        if n.has_target && !edges.iter().any(|e| e.to == n.id) {
            issues.push(format!("{} has no incoming connection.", n.label));
        }
        if n.has_source && !edges.iter().any(|e| e.from == n.id) {
            issues.push(format!("{} has no outgoing connection.", n.label));
        }
    }
    for e in edges {
        if !nodes.iter().any(|n| n.id == e.from) || !nodes.iter().any(|n| n.id == e.to) {
            issues.push("A connection references a removed step.".into());
        }
    }
    issues
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_is_connected_and_deletion_is_diagnosed() {
        let (mut nodes, edges) = sample();
        assert!(draft_issues(&nodes, &edges).is_empty());
        nodes.remove(0);
        let issues = draft_issues(&nodes, &edges);
        assert!(issues.iter().any(|s| s.contains("entry trigger")));
        assert!(issues.iter().any(|s| s.contains("removed step")));
    }
}
