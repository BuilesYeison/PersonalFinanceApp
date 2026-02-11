import type { AccountInfoDto } from "./account_info.dto";
import type { CategoryDto } from "./category.dto";

export type RecordDto = {
  id: string;
  type: string;
  amount: number;
  currency: string | undefined | null;
  timestamp: number;
  category: CategoryDto | null | undefined;
  account: AccountInfoDto | null | undefined;
  to_account: AccountInfoDto | null | undefined;
  description: string | null | undefined;
};
