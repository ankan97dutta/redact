use redact_domain::types::Confidence;

pub struct RuleSpec {
    pub id: &'static str,
    pub category: &'static str,
    pub pattern: &'static str,
    pub confidence: Confidence,
}

/// MVP baseline built-ins (T036). Names/addresses arrive in T082.
pub const RULE_SPECS: &[RuleSpec] = &[
    RuleSpec {
        id: "pan",
        category: "pan",
        // Synthetic / test-friendly PAN shape (5 letters + 4 digits + 1 letter)
        pattern: r"\b[A-Z]{5}[0-9]{4}[A-Z]\b",
        confidence: Confidence::High,
    },
    RuleSpec {
        id: "aadhaar",
        category: "aadhaar",
        pattern: r"\b[2-9]{1}[0-9]{3}\s?[0-9]{4}\s?[0-9]{4}\b",
        confidence: Confidence::Medium,
    },
    RuleSpec {
        id: "gstin",
        category: "gstin",
        pattern: r"\b[0-9]{2}[A-Z]{5}[0-9]{4}[A-Z]{1}[1-9A-Z]{1}Z[0-9A-Z]{1}\b",
        confidence: Confidence::High,
    },
    RuleSpec {
        id: "ifsc",
        category: "ifsc",
        pattern: r"\b[A-Z]{4}0[A-Z0-9]{6}\b",
        confidence: Confidence::High,
    },
    RuleSpec {
        id: "bank_account",
        category: "bank",
        pattern: r"\b[0-9]{9,18}\b",
        confidence: Confidence::Low,
    },
    RuleSpec {
        id: "phone",
        category: "phone",
        pattern: r"\b(?:\+91[\-\s]?)?[6-9][0-9]{9}\b",
        confidence: Confidence::Medium,
    },
    RuleSpec {
        id: "email",
        category: "email",
        pattern: r"\b[A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,}\b",
        confidence: Confidence::High,
    },
];
