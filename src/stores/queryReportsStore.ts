import { ref } from "vue";
import { defineStore } from "pinia";
import { QueryReportsRequest } from "@/services/reportApi";
import { defaultPage } from "@/utils/constants";

export const useQueryReportsStore = defineStore("queryReports", () => {
  // state
  const searchParams = ref<QueryReportsRequest>({ ...defaultPage });

  // actions
  function setPage(page: number) {
    if (page >= 1) searchParams.value = { ...searchParams.value, page };
  }

  function setSearchParams(params: QueryReportsRequest) {
    const cleanedParams: Record<string, string | number> = { ...defaultPage };
    (Object.entries(params) as [keyof QueryReportsRequest, string | number | undefined][]).forEach(([key, value]) => {
      if (value !== undefined && value !== "" && value !== null) {
        cleanedParams[key] = value;
      }
    });
    searchParams.value = cleanedParams as QueryReportsRequest;
  }

  function resetSearchParams() {
    searchParams.value = { ...defaultPage };
  }

  return {
    searchParams,
    setPage,
    setSearchParams,
    resetSearchParams,
  };
});
