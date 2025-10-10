import { defineStore } from "pinia";
import { ref } from "vue";
import { defaultPage } from "@/utils/constants";
import { QueryNewsRequest } from "@/services/newsApi";

export const useQueryNewsStore = defineStore("queryNews", () => {
  // state
  const searchParams = ref<QueryNewsRequest>({ ...defaultPage });

  // actions
  function setPage(page: number) {
    if (page >= 1) searchParams.value = { ...searchParams.value, page };
  }

  function setSearchParams(params: QueryNewsRequest) {
    const cleanedParams: Record<string, string | number> = { ...defaultPage };
    (Object.entries(params) as [keyof QueryNewsRequest, string | number | undefined][]).forEach(([key, value]) => {
      if (value !== undefined && value !== "" && value !== null) {
        cleanedParams[key] = value;
      }
    });
    searchParams.value = cleanedParams;
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
