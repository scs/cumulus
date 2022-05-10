#!/bin/bash

# Create `WeightInfo` implementations for all the pallets and store it in the weight module of the `integritee-runtime`.

INTEGRITEE_RUNTIME_WEIGHT_DIR=polkadot-parachains/integritee-runtime/src/weights
COLLATOR=./target/release/integritee-collator

mkdir -p $INTEGRITEE_RUNTIME_WEIGHT_DIR

pallets=(
  "frame_system" \
  "pallet_balances" \
  "pallet_collective_council" \
  "pallet_collective_technical_committee" \
  "pallet_democracy" \
  "pallet_timestamp" \
  "pallet_vesting" \
  "pallet_teerex" \
  "pallet_claims" \
  "pallet_migration" \
  "pallet_multisig" \
  "pallet_proxy" \
  "pallet_scheduler" \
  "pallet_teeracle" \
  "pallet_treasury" \
  "pallet_utility" \
)

for pallet in ${pallets[*]}; do
  echo benchmarking "$pallet"...

  $COLLATOR \
  benchmark pallet \
  --chain=integritee-rococo-local-dev \
  --steps=50 \
  --repeat=20 \
  --pallet="$pallet" \
  --extrinsic="*" \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output=./$INTEGRITEE_RUNTIME_WEIGHT_DIR/"$pallet".rs \

done
