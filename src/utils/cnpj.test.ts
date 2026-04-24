import { describe, expect, it } from "vitest";
import { getCnpjValidationState, isValidCnpj, sanitizeCnpj } from "./cnpj";

describe("cnpj utils", () => {
  it("sanitizes masked values", () => {
    expect(sanitizeCnpj("11.222.333/0001-81")).toBe("11222333000181");
  });

  it("accepts a valid masked CNPJ", () => {
    expect(isValidCnpj("11.222.333/0001-81")).toBe(true);
  });

  it("accepts a valid unmasked CNPJ", () => {
    expect(isValidCnpj("11222333000181")).toBe(true);
  });

  it("rejects an invalid CNPJ", () => {
    expect(isValidCnpj("11.222.333/0001-82")).toBe(false);
  });

  it("rejects repeated digits", () => {
    expect(isValidCnpj("00.000.000/0000-00")).toBe(false);
  });

  it("reports empty state", () => {
    expect(getCnpjValidationState("")).toBe("empty");
  });

  it("reports valid state", () => {
    expect(getCnpjValidationState("11222333000181")).toBe("valid");
  });

  it("reports invalid state", () => {
    expect(getCnpjValidationState("123")).toBe("invalid");
  });
});
