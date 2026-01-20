export type AccountInfoDto = {
  name: string;
  balance: number;
  account_type: string | null | undefined;
  currency: string | null | undefined;
  initial_balance: number | null | undefined;
  credit_limit: number | null | undefined;
};
