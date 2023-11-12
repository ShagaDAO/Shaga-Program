.PHONY: build_program solita_build upgrade_shaga

git: 
	git add .
	git commit -m "$m"
	git push -u origin

pre_git: build_program deploy_shaga upgrade_idl_shaga

build_program:
	anchor build

build_shaga_program:
	anchor build -p shaga 

deploy_shaga:
	anchor deploy --program-keypair target/deploy/shaga-keypair.json --provider.cluster devnet --provider.wallet app/test_keypairs/0.json -p shaga 

deploy_idl_shaga:
	anchor idl init --filepath target/idl/shaga.json --provider.cluster devnet --provider.wallet app/test_keypairs/0.json HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1

upgrade_idl_shaga:
	anchor idl upgrade --filepath target/idl/shaga.json --provider.cluster devnet --provider.wallet app/test_keypairs/0.json HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1

upgrade_shaga:
	anchor upgrade --program-id HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1 --provider.cluster devnet --provider.wallet app/test_keypairs/0.json target/deploy/shaga.so

solita_build:
	cd app/shaga && yarn node_solita && cd ..

transfer_to_test_keypairs:
	solana transfer app/test_keypairs/5.json 2 --allow-unfunded-recipient
	solana transfer app/test_keypairs/6.json 2 --allow-unfunded-recipient

balance_test_keypairs:
	solana balance -k app/test_keypairs/0.json
	solana balance -k app/test_keypairs/1.json
	solana balance -k app/test_keypairs/2.json
	solana balance -k app/test_keypairs/3.json
	solana balance -k app/test_keypairs/4.json
	solana balance -k app/test_keypairs/5.json
	solana balance -k app/test_keypairs/6.json

run_ts:
	cd app/shaga && yarn start $m && cd ../.. 