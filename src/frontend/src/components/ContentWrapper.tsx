import { Footer } from "@/components/Footer";
import CalimeroLogo from "@/assets/calimero-logo.svg";
import ICPLogo from "@/assets/icp-logo.svg";
interface ContentWrapperProps {
  children: React.ReactNode;
}

export const ContentWrapper = ({ children }: ContentWrapperProps) => {
  return (
    <div className="min-h-screen w-full bg-pink-500">
      {/* Navigation Bar */}
      <nav className="flex justify-between px-2 py-4 bg-green-500">
        <div className="relative flex items-center gap-2">
          <img
            src={CalimeroLogo}
            alt="Calimero Admin Dashboard Logo"
            className="h-[43.3px] w-[160px]"
          />
          <img src={ICPLogo} alt="ICP Logo" className="h-[43.3px] w-[160px]" />
        </div>
      </nav>

      {/* Main Content */}
      <div className="flex flex-col justify-center">{children}</div>
      <Footer />
    </div>
  );
};

export default ContentWrapper;
