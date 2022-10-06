import { ApiPromise } from "@polkadot/api";
import { u128 } from "@polkadot/types-codec";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import {
  getTransferCallKaruraPicasso,
  getTransferCallKusamaPicasso,
  getTransferCallPicassoKarura,
  getTransferCallPicassoKusama
} from "@/defi/polkadot/pallets/xcmp";
import { toChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

export async function getApiCallAndSigner(
  api: ApiPromise,
  targetAccountAddress: string,
  amountToTransfer: u128,
  feeItemId: number | null,
  signerAddress: string,
  targetParachainId: number,
  from: SubstrateNetworkId,
  to: SubstrateNetworkId,
  hasFeeItem: boolean,
  weight: BigNumber
) {
  switch (`${from}-${to}`) {
    case "picasso-kusama":
      return getTransferCallPicassoKusama(
        api,
        targetAccountAddress,
        amountToTransfer,
        feeItemId,
        signerAddress,
        hasFeeItem,
        weight
      );
    case "picasso-karura":
      return getTransferCallPicassoKarura(
        api,
        targetParachainId,
        targetAccountAddress,
        hasFeeItem,
        signerAddress,
        amountToTransfer,
        feeItemId
      );
    case "kusama-picasso":
      return getTransferCallKusamaPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        amountToTransfer,
        signerAddress
      );
    case "karura-picasso":
      return getTransferCallKaruraPicasso(
        api,
        targetParachainId,
        targetAccountAddress,
        signerAddress,
        amountToTransfer
      );
    default:
      throw new Error("Invalid network");
  }
}

export function getAmountToTransfer({
  balance,
  amount,
  existentialDeposit,
  keepAlive,
  api
}: {
  balance: BigNumber;
  amount: BigNumber;
  existentialDeposit: BigNumber;
  keepAlive: boolean;
  api: ApiPromise;
}): u128 {
  const isExistentialDepositImportant = balance
    .minus(amount)
    .lte(existentialDeposit);
  const isZeroAmount =
    keepAlive &&
    isExistentialDepositImportant &&
    amount.minus(existentialDeposit).lte(0);
  return api.createType(
    "u128",
    toChainIdUnit(
      keepAlive && isExistentialDepositImportant && !isZeroAmount
        ? amount.minus(existentialDeposit)
        : amount
    ).toString()
  );
}
