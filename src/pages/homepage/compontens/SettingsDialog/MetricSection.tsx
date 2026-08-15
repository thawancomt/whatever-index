import { Badge } from "@/components/ui/badge";
import { useMetrics } from "@/stores/metrics.store";
import { SettingsSection, SettingsSectionList, SettingsSectionListItem } from "./SettingsDialog";

export default function MetricSection() {
    const { total_files } = useMetrics();

    return (
        <SettingsSection title="Metrics">
            <SettingsSectionList>
                <SettingsSectionListItem label="Total files indexed">
                    <Badge variant={"ghost"} className="font-bold">{total_files}</Badge>
                </SettingsSectionListItem>
                <SettingsSectionListItem label="Last scan">
                    <Badge variant={"ghost"} className="font-bold">Today</Badge>
                </SettingsSectionListItem>
            </SettingsSectionList>
        </SettingsSection>
    )
}