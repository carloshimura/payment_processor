#[cfg(test)]
mod tests {
    use payments_engine::data_models::TransactionId;
    use payments_engine::errors::TransactionReaderError;
    use payments_engine::read_and_parse_transactions;

    #[test]
    fn test_basic_payments() {
        let mut output = Vec::new();
        assert!(
            read_and_parse_transactions("test_resources/simple_transactions.csv", &mut output)
                .is_ok()
        );
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,1.5,0,1.5,false");
        assert_eq!(string_lines[2], "2,2,0,2,false");
    }

    #[test]
    fn test_contested_payments() {
        let mut output = Vec::new();
        assert!(read_and_parse_transactions(
            "test_resources/contest_transactions.csv",
            &mut output
        )
        .is_ok());
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,95,0,95,false");
        assert_eq!(string_lines[2], "2,200,0,200,true");
    }

    #[test]
    fn test_unresolved_contest() {
        let mut output = Vec::new();
        assert!(
            read_and_parse_transactions("test_resources/contest_unresolved.csv", &mut output)
                .is_ok()
        );
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,100,-5,95,false");
    }

    #[test]
    fn test_unresolved_deposit_contest() {
        let mut output = Vec::new();
        assert!(read_and_parse_transactions(
            "test_resources/contest_unresolved_deposit.csv",
            &mut output
        )
        .is_ok());
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,-5,100,95,false");
    }

    #[test]
    fn test_unresolved_contest_withdrawal() {
        let mut output = Vec::new();
        assert!(read_and_parse_transactions(
            "test_resources/contest_unresolved_withdrawal.csv",
            &mut output
        )
        .is_ok());
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,25,50,75,false");
    }

    #[test]
    fn test_failed_withdrawal() {
        let mut output = Vec::new();
        assert!(
            read_and_parse_transactions("test_resources/failed_withdrawal.csv", &mut output)
                .is_ok()
        );
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,100,0,100,false");
    }

    #[test]
    fn test_accepted_withdrawal() {
        let mut output = Vec::new();
        assert!(
            read_and_parse_transactions("test_resources/accepted_withdrawal.csv", &mut output)
                .is_ok()
        );
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,0,0,0,false");
    }

    #[test]
    fn test_duplicated_entries() {
        let mut output = Vec::new();
        assert!(read_and_parse_transactions(
            "test_resources/duplicated_transactions.csv",
            &mut output
        )
        .is_ok());
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,50,0,50,false");
    }

    #[test]
    fn test_duplicated_contest() {
        let mut output = Vec::new();
        assert!(
            read_and_parse_transactions("test_resources/duplicated_contest.csv", &mut output)
                .is_ok()
        );
        let lines = String::from_utf8(output).expect("Failed to convert output to string");
        let mut string_lines = lines.split('\n').collect::<Vec<&str>>();
        assert!(!string_lines.is_empty());
        assert_eq!(string_lines[0], "client, available, held, total, locked");
        string_lines.remove(0);
        string_lines.sort();
        assert_eq!(string_lines[0], "");
        assert_eq!(string_lines[1], "1,100,0,100,true");
    }

    #[test]
    fn test_invalid_withdrawal_records() {
        let mut output = Vec::new();
        assert!(matches!(
            read_and_parse_transactions(
                "test_resources/invalid_withdrawal_records.csv",
                &mut output
            ),
            Err(TransactionReaderError::InvalidDeposit(TransactionId {
                value: 2
            }))
        ));
    }

    #[test]
    fn test_invalid_records() {
        let mut output = Vec::new();
        assert!(matches!(
            read_and_parse_transactions("test_resources/invalid_records.csv", &mut output),
            Err(TransactionReaderError::CSVReaderError(_))
        ));
    }
}
