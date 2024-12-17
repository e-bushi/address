use address::state::AddressInfo;

mod tests {
    use super::*;
    use borsh::BorshDeserialize;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signer}, system_program, transaction::Transaction, msg
    };


    #[tokio::test]
    async fn test_create_address() {
        msg!("Testing create address");
        let program_id = Pubkey::new_unique();
        let (banks_client, payer, recent_blockhash) = ProgramTest::new(
            "address",
            program_id,
            processor!(address::processor::process_instruction),
        )
        .start()
        .await;

        let address_keypair = Keypair::new();
        let initial_address = AddressInfo {
            name: "John Doe".to_string(),
            house_number: 123,
            street: "33 Mapleton Road".to_string(),
            city: "Auckland".to_string(),
        };

        msg!("Initial address: {:?}", initial_address);
        let init_instruction_data: Vec<u8> = initial_address.try_to_vec().unwrap();


        let initialize_instruction = Instruction::new_with_bytes(
            program_id, 
            &init_instruction_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_keypair.pubkey(), true),
                AccountMeta::new(system_program::id(), false),
            ]
        );

        let mut transaction = Transaction::new_with_payer(
            &[initialize_instruction], 
            Some(&payer.pubkey())
        );

        transaction.sign(&[&payer, &address_keypair], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        let account = banks_client
        .get_account(address_keypair.pubkey())
        .await
        .expect("Failed to get account");

        if let Some(account_data) = account {
            let new_address: AddressInfo = AddressInfo::try_from_slice(&account_data.data)
            .expect("Failed to deserialize address");

            assert_eq!(new_address, initial_address);
            println!(
                "✅ Address initialized successfully with value: {}",
                new_address
            );
        }


    }
}