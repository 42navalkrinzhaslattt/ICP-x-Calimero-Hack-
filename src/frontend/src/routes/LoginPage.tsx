import { ClientLogin } from "@calimero-network/calimero-client";
import { useNavigate } from "react-router-dom";
import { getApplicationId, getNodeUrl } from "@/utils/node";
import {
  clearApplicationIdFromLocalStorage,
  clearNodeUrlFromLocalStorage,
} from "@/utils/storage";
import ContentWrapper from "@/components/ContentWrapper";

export default function LoginPage() {
  const navigate = useNavigate();
  function onSetupClick() {
    clearNodeUrlFromLocalStorage();
    clearApplicationIdFromLocalStorage();
    navigate("/");
  }
  return (
    <ContentWrapper>
      <div className="relative flex h-screen w-full bg-gray-900">
        <div className="flex h-full w-full flex-col items-center justify-center">
          <div className="rounded-lg bg-gray-800 p-8">
            <div className="flex items-center justify-center gap-3 px-14">
              <div className="text-2xl font-bold text-pink-500 bg-white border-3 border-pink-500 rounded-lg p-2">
                {`${" B I L L O "}`}
              </div>
            </div>
            <div className="bg-gray-800">
              <ClientLogin
                getNodeUrl={getNodeUrl}
                getApplicationId={getApplicationId}
                sucessRedirect={() => navigate("/home")}
              />
            </div>
          </div>

          <button
            onClick={onSetupClick}
            // className="mt-4 cursor-pointer p-4 text-white hover:text-gray-300"
            className="mt-4 rounded-lg border border-gray-600 px-6 py-2 text-gray-400 transition-all duration-200 hover:border-pink-500 hover:text-pink-500"
          >
            Return to setup
          </button>
        </div>
      </div>
    </ContentWrapper>
  );
}
