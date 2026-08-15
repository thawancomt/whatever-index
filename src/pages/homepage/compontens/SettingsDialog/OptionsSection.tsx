import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Switch } from "@/components/ui/switch";
import { AppSettings, useSettings, useUpdateSettings } from "@/hooks/useSettings";
import { SettingsSection, SettingsSectionList, SettingsSectionListItem } from "./SettingsDialog";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TauriResponse } from "@/domain/types/error_response";
import { useFeatures } from "@/stores/feature.store";

export default function OptionsSection() {
    const { data: settings } = useSettings();

  const { mutateAsync: updateSettingsAction } = useUpdateSettings();

  const { ocr } = useFeatures()

    const updateSettings = async (key: keyof AppSettings, value: boolean) => {
        if (!settings) return;
        const patch = {
            ...settings,
            [key]: value
        } satisfies AppSettings;
        await updateSettingsAction(patch)
    }

  useEffect(() => {
    const check_ocr_models = async () => {

      try {
        await invoke<TauriResponse>("get_ocr_models_status");
        useFeatures.setState({
          ocr : true
        })
      } catch (e) {}

    }
    check_ocr_models()
  }, [settings])

    return (
        <SettingsSection title="Options">
            <SettingsSectionList>
                <SettingsSectionListItem label="Auto Re-scan" className="w-full  justify-between">
                    <Switch checked={settings?.auto_scan || false} onCheckedChange={(e) => {
                        updateSettings("auto_scan", e)
                    }} />
                </SettingsSectionListItem>

                <SettingsSectionListItem label={
                    <span className="inline-flex items-center pr-1">
                        Index images
                        {(settings?.index_images && !ocr) && (
                            <Badge variant={"destructive"} className="ml-2">OCR missing</Badge>
                        )}
                    </span>
                } className="w-full  justify-between">
                    <Switch checked={settings?.index_images || false} disabled={!ocr} onCheckedChange={(e) => {
                        updateSettings("index_images", e)
                    }} />
                </SettingsSectionListItem>

                {
                    (settings?.index_images && !ocr) && (
                        <SettingsSectionListItem label="Aditional content for OCR" className="w-full text-xs! flex-col items-start! pl-2 p-1 rounded-lg">
                            <Progress value={0.4} max={1} className="w-full " />
                        </SettingsSectionListItem>
                    )
                }

                <SettingsSectionListItem label="Scan audio files" className="w-full  justify-between">
                    <Switch checked={settings?.index_audio || false} onCheckedChange={(e) => {
                        updateSettings("index_audio", e)
                    }} />
                </SettingsSectionListItem>
            </SettingsSectionList>
        </SettingsSection>
    )
}
