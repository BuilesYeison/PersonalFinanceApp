export type CreateRecordDto = {
  type: "expense" | "income" | "transfer";
  amount: number;
  account_id: string;
  to_account_id?: string;
  category_id?: string;
  description?: string;
  timestamp: number;
};
