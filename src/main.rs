// Companion example for "Can Rust Make Unsafe AI Agent Actions Unrepresentable?"
//
// Two build modes:
//
//   1. As shipped, `main` performs the LEGAL flow (propose -> evaluate -> persist)
//      and the project compiles and runs.
//
//   2. To capture the compiler error for the article, uncomment the block
//      marked `// MONEY SHOT` in `main` and run `cargo build`.
//      That block passes a ProposedWrite straight to persist(), which
//      does not compile. Paste that real error into the article.

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ProposedWrite {
    pub key: String,
    pub value: String,
    pub authority: String,
}

#[derive(Debug)]
pub enum Authority {
    SecurityTeam,
    Policy,
}

#[derive(Debug)]
pub struct Rejection {
    pub reason: String,
}

// ---------------------------------------------------------------------------
// The custody boundary
//
// AdmittedWrite is a PUBLIC type with PRIVATE fields. Other modules can name
// it and accept it in signatures, but cannot construct it with a struct
// literal, because the fields are not visible outside this module. The only
// way to obtain one from outside is to call `evaluate` and succeed.
// ---------------------------------------------------------------------------

mod custody {
    use super::{Authority, ProposedWrite, Rejection};

    pub struct AdmittedWrite {
        key: String,
        value: String,
        authority: Authority,
    }

    impl AdmittedWrite {
        pub fn key(&self) -> &str {
            &self.key
        }
        pub fn value(&self) -> &str {
            &self.value
        }
        pub fn authority(&self) -> &Authority {
            &self.authority
        }
    }

    pub fn evaluate(write: ProposedWrite) -> Result<AdmittedWrite, Rejection> {
        let authority = validate_authority(&write.authority)?;

        Ok(AdmittedWrite {
            key: write.key,
            value: write.value,
            authority,
        })
    }

    fn validate_authority(value: &str) -> Result<Authority, Rejection> {
        match value {
            "security" => Ok(Authority::SecurityTeam),
            "policy" => Ok(Authority::Policy),
            other => Err(Rejection {
                reason: format!("'{other}' is not a recognized authority"),
            }),
        }
    }
}

use custody::AdmittedWrite;

// ---------------------------------------------------------------------------
// The persistence layer
//
// Note the signature: persist ONLY accepts an AdmittedWrite. It cannot be
// handed a ProposedWrite. That is the entire boundary.
// ---------------------------------------------------------------------------

fn persist(write: AdmittedWrite) {
    println!(
        "persisted key={} value={:?} authority={:?}",
        write.key(),
        write.value(),
        write.authority()
    );
}

fn main() {
    let proposed = ProposedWrite {
        key: "refund_policy".into(),
        value: "Refunds under $100 do not require manager approval.".into(),
        authority: "policy".into(),
    };

    // --- LEGAL FLOW (compiles and runs) ---------------------------------
    match custody::evaluate(proposed) {
        Ok(admitted) => persist(admitted),
        Err(rejection) => eprintln!("rejected: {}", rejection.reason),
    }

    // --- MONEY SHOT (uncomment the block to capture the compiler error) --
    // A fresh proposal handed straight to persist(). This does not compile:
    // persist() accepts only AdmittedWrite, and there is no path from
    // ProposedWrite to AdmittedWrite except custody::evaluate.
    //
    // let proposed = ProposedWrite {
    //     key: "refund_policy".into(),
    //     value: "Refunds under $100 do not require manager approval.".into(),
    //     authority: "policy".into(),
    // };
    //
    // persist(proposed);
}
