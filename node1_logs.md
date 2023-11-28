ganesholi@Ganeshs-MacBook-Air xcav-parachain % ./target/release/parachain-template-node --collator \                                        
  --chain raw-parachain-chainspec.json \
  --base-path ./tmp/node01 \
  --port 50333 \
  --rpc-port 8855 \
  -- \
  --execution wasm \
  --chain ./rococo.json \
  --port 50343 \
  --rpc-port 9988
2023-11-23 06:59:28 Parachain Collator Template    
2023-11-23 06:59:28 ✌️  version 1.1.0-c0948d85228    
2023-11-23 06:59:28 ❤️  by Anonymous, 2020-2023    
2023-11-23 06:59:28 📋 Chain specification: Development Local Testnet    
2023-11-23 06:59:28 🏷  Node name: highfalutin-effect-1038    
2023-11-23 06:59:28 👤 Role: AUTHORITY    
2023-11-23 06:59:28 💾 Database: RocksDb at ./tmp/node01/chains/dev_local_testnet/db/full    
CLI parameter `--execution` has no effect anymore and will be removed in the future!
2023-11-23 06:59:29 Parachain id: Id(4323)    
2023-11-23 06:59:29 Parachain Account: 5Ec4AhPYkcPrqb2Lo5ycwxiUdDCZ7hnLttj9By7CeKApcJrP    
2023-11-23 06:59:29 Parachain genesis state: 0x000000000000000000000000000000000000000000000000000000000000000000ece0e962b721ba11023c598d478276669466762a5ba789e15cd42565c0c3d26c03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c11131400    
2023-11-23 06:59:29 Is collating: yes    
2023-11-23 06:59:33 [Relaychain] 🏷  Local node identity is: 12D3KooWFwYQoge7H2BVDLm3Cqj6ueFRNnNE6xKErpAShZaHFEbg    
2023-11-23 06:59:33 [Relaychain] 💻 Operating system: macos    
2023-11-23 06:59:33 [Relaychain] 💻 CPU architecture: aarch64    
2023-11-23 06:59:33 [Relaychain] 📦 Highest known block at #7641    
2023-11-23 06:59:33 [Relaychain] 〽️ Prometheus exporter started at 127.0.0.1:9616    
2023-11-23 06:59:33 [Relaychain] Running JSON-RPC server: addr=127.0.0.1:9988, allowed origins=["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org"]    
2023-11-23 06:59:33 [Relaychain] 🏁 CPU score: 608.25 MiBs    
2023-11-23 06:59:33 [Relaychain] 🏁 Memory score: 27.60 GiBs    
2023-11-23 06:59:33 [Relaychain] 🏁 Disk score (seq. writes): 1.19 GiBs    
2023-11-23 06:59:33 [Relaychain] 🏁 Disk score (rand. writes): 271.92 MiBs    
2023-11-23 06:59:33 [Relaychain] Starting with an empty approval vote DB.
2023-11-23 06:59:33 [Parachain] 🏷  Local node identity is: 12D3KooWHSuB7Xnj1HhMzgSBQqMy4byVqUQwbhZr6b5T7TtGUwsE    
2023-11-23 06:59:33 [Parachain] 💻 Operating system: macos    
2023-11-23 06:59:33 [Parachain] 💻 CPU architecture: aarch64    
2023-11-23 06:59:33 [Parachain] 📦 Highest known block at #0    
2023-11-23 06:59:33 [Parachain] 〽️ Prometheus exporter started at 127.0.0.1:9615    
2023-11-23 06:59:33 [Parachain] Running JSON-RPC server: addr=127.0.0.1:8855, allowed origins=["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org"]    
2023-11-23 06:59:33 [Parachain] 🏁 CPU score: 608.25 MiBs    
2023-11-23 06:59:33 [Parachain] 🏁 Memory score: 27.60 GiBs    
2023-11-23 06:59:33 [Parachain] 🏁 Disk score (seq. writes): 1.19 GiBs    
2023-11-23 06:59:33 [Parachain] 🏁 Disk score (rand. writes): 271.92 MiBs    
2023-11-23 06:59:33 [Parachain] ⚠️  The hardware does not meet the minimal requirements for role 'Authority'.    
2023-11-23 06:59:33 [Relaychain] discovered: 12D3KooWHSuB7Xnj1HhMzgSBQqMy4byVqUQwbhZr6b5T7TtGUwsE /ip4/192.168.1.66/tcp/50333    
2023-11-23 06:59:33 [Parachain] discovered: 12D3KooWFwYQoge7H2BVDLm3Cqj6ueFRNnNE6xKErpAShZaHFEbg /ip4/192.168.1.66/tcp/50343/ws    
2023-11-23 06:59:35 [Relaychain] 🔍 Discovered new external address for our node: /ip4/27.34.99.147/tcp/50343/ws/p2p/12D3KooWFwYQoge7H2BVDLm3Cqj6ueFRNnNE6xKErpAShZaHFEbg    
2023-11-23 06:59:36 [Relaychain] 🔍 Discovered new external address for our node: /ip6/2400:1a00:bb10:f953:6d06:5643:3eb4:bc3e/tcp/50343/ws/p2p/12D3KooWFwYQoge7H2BVDLm3Cqj6ueFRNnNE6xKErpAShZaHFEbg    
2023-11-23 06:59:38 [Relaychain] ⚙️  Syncing, target=#8000785 (8 peers), best: #8267 (0x0e9b…3699), finalized #8192 (0x2e18…3447), ⬇ 981.0kiB/s ⬆ 66.0kiB/s    
2023-11-23 06:59:38 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 0.5kiB/s ⬆ 0.3kiB/s    
2023-11-23 06:59:43 [Relaychain] ⚙️  Syncing 241.1 bps, target=#8000786 (8 peers), best: #9474 (0x3c43…fbb9), finalized #9445 (0x6890…0942), ⬇ 3.2MiB/s ⬆ 54.9kiB/s    
2023-11-23 06:59:43 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 65 B/s ⬆ 11 B/s    
2023-11-23 06:59:48 [Relaychain] 🔍 Discovered new external address for our node: /ip6/2400:1a00:bb10:f953:cdd:a758:d789:3203/tcp/50343/ws/p2p/12D3KooWFwYQoge7H2BVDLm3Cqj6ueFRNnNE6xKErpAShZaHFEbg    
2023-11-23 06:59:48 [Relaychain] ⚙️  Syncing 152.0 bps, target=#8000787 (9 peers), best: #10235 (0xa146…2bf6), finalized #10022 (0x8666…c878), ⬇ 2.5MiB/s ⬆ 16.6kiB/s    
2023-11-23 06:59:48 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 5 B/s ⬆ 41 B/s    
2023-11-23 06:59:51 [Relaychain] CurrentBlockRandomness did not provide entropy    
2023-11-23 06:59:53 [Relaychain] ⚙️  Syncing 125.7 bps, target=#8000787 (11 peers), best: #10864 (0x846d…ed81), finalized #10752 (0x7701…939a), ⬇ 771.3kiB/s ⬆ 27.9kiB/s    
2023-11-23 06:59:53 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 18 B/s ⬆ 54 B/s    
2023-11-23 06:59:58 [Relaychain] ⚙️  Syncing 107.3 bps, target=#8000788 (11 peers), best: #11401 (0xcaa0…d86f), finalized #11264 (0xcdd7…ff67), ⬇ 68.2kiB/s ⬆ 6.1kiB/s    
2023-11-23 06:59:58 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 5 B/s ⬆ 41 B/s    
2023-11-23 07:00:03 [Relaychain] ⚙️  Syncing 93.4 bps, target=#8000789 (11 peers), best: #11868 (0x792b…306d), finalized #11776 (0x4b05…2171), ⬇ 125.9kiB/s ⬆ 8.9kiB/s    
2023-11-23 07:00:03 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 0 ⬆ 0    
2023-11-23 07:00:08 [Relaychain] ⚙️  Syncing 91.6 bps, target=#8000790 (10 peers), best: #12326 (0xcfc1…9036), finalized #12288 (0x4dc9…c760), ⬇ 3.8MiB/s ⬆ 10.9kiB/s    
2023-11-23 07:00:08 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 18 B/s ⬆ 54 B/s    
2023-11-23 07:00:13 [Relaychain] ⚙️  Syncing 94.7 bps, target=#8000791 (10 peers), best: #12800 (0xadd8…7385), finalized #12364 (0xd7e9…a5fb), ⬇ 1.7MiB/s ⬆ 5.1kiB/s    
2023-11-23 07:00:13 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 0 ⬆ 0    
2023-11-23 07:00:18 [Relaychain] ⚙️  Syncing 81.8 bps, target=#8000792 (10 peers), best: #13209 (0xe61b…6885), finalized #12957 (0x93e6…9f85), ⬇ 564.0kiB/s ⬆ 8.2kiB/s    
2023-11-23 07:00:18 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 5 B/s ⬆ 41 B/s    
2023-11-23 07:00:23 [Relaychain] ⚙️  Syncing 87.5 bps, target=#8000792 (10 peers), best: #13647 (0x3f48…47d3), finalized #13550 (0x98e2…b72b), ⬇ 654.2kiB/s ⬆ 4.7kiB/s    
2023-11-23 07:00:23 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 18 B/s ⬆ 54 B/s    
2023-11-23 07:00:28 [Relaychain] ⚙️  Syncing 92.3 bps, target=#8000793 (10 peers), best: #14109 (0xb8d8…89c6), finalized #13824 (0xf067…8214), ⬇ 61.8kiB/s ⬆ 6.7kiB/s    
2023-11-23 07:00:28 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 0 ⬆ 0    
2023-11-23 07:00:33 [Relaychain] ⚙️  Syncing 90.8 bps, target=#8000794 (10 peers), best: #14563 (0x128a…f800), finalized #14336 (0x7ee1…1e0d), ⬇ 37.5kiB/s ⬆ 8.2kiB/s    
2023-11-23 07:00:33 [Parachain] 💤 Idle (0 peers), best: #0 (0x9761…dc19), finalized #0 (0x9761…dc19), ⬇ 0.5kiB/s ⬆ 0.3kiB/s  