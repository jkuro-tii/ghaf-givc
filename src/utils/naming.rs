use anyhow::*;

pub fn format_vm_name(name: &String) -> String {
    format!("microvm@{name}-vm.service")
}

pub fn format_service_name(name: &String) -> String {
    format!("givc-{}-vm.service", name)
}

// FIXME: rewrite as `pub fn parse_service_name(name: &str) -> anyhow::Result<&str>`
pub fn parse_service_name(name: &String) -> anyhow::Result<String> {
    name.strip_suffix("-vm.service")
        .and_then(|name| name.strip_prefix("givc-"))
        .map(str::to_string)
        .ok_or_else(|| anyhow!("Doesn't know how to parse VM name: {name}"))
}

// From `agent` code, ported for future
// FIXME: rewrite as `parse_application_name(name: &str) -> anyhow::Result<(&str, i32)>`
pub fn parse_application_name(name: &String) -> anyhow::Result<(String, i32)> {
    if let Some(name_no_suffix) = name.strip_suffix(".service") {
        if let Some((left, right)) = name_no_suffix.rsplit_once("@") {
            let num = right
                .parse::<i32>()
                .with_context(|| format!("While parsing number part of {}", name))?;
            return Ok((left.to_string(), num));
        }
    };
    bail!("App name {} not it app@<number>.service format", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_service_name() -> Result<()> {
        let good = parse_service_name(&String::from("givc-good-vm.service"))?;
        assert_eq!(good, "good");

        let bad = parse_service_name(&String::from("just-a.service"));
        let err = bad.unwrap_err();
        assert_eq!(
            format!("{}", err.root_cause()),
            "Doesn't know how to parse VM name: just-a.service"
        );
        Ok(())
    }

    #[test]
    fn test_parse_application_name() -> Result<()> {
        let good = parse_application_name(&String::from("good-app@42.service"))?;
        assert_eq!(good, (String::from("good-app"), 42));

        let bad = parse_application_name(&String::from("just-a.service"));
        let err = bad.unwrap_err();
        assert_eq!(
            format!("{}", err.root_cause()),
            "App name just-a.service not it app@<number>.service format"
        );

        Ok(())
    }
}
