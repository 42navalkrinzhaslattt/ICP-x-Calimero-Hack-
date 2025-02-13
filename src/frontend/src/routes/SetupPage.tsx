import { useNavigate } from "react-router-dom";
import { SetupModal } from "@calimero-network/calimero-client";
import { ContentWrapper } from "@/components/ContentWrapper";
import { getNodeUrl, getApplicationId } from "@/utils/node";
import {
  setNodeUrlToLocalStorage,
  setApplicationIdToLocalStorage,
} from "@/utils/storage";

export default function SetupPage() {
  const navigate = useNavigate();

  return (
    <ContentWrapper>
      <SetupModal
        successRoute={() => navigate("/login")}
        getNodeUrl={getNodeUrl}
        setNodeUrl={setNodeUrlToLocalStorage}
        setApplicationId={setApplicationIdToLocalStorage}
        getApplicationId={getApplicationId}
      />
    </ContentWrapper>
  );
}
