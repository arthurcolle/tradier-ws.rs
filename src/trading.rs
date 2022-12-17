mod trading {
  pub struct Strategy {
    recipe: Vec<Rule>
  }

  pub enum RuleType {
    EntryRule,
    ExitRule
  }

  pub struct Rule {
    label: String,
    rule_type: RuleType,
    description: String
  }
}