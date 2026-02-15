import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { usePresetStore } from "@/stores/presetStore";
import type { Preset } from "@/types/preset";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import { Badge } from "@/components/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { ArrowLeft, Plus, Play, Trash2, Save } from "lucide-react";

export function PresetManager() {
  const navigate = useNavigate();
  const { presets, addPreset, deletePreset, setCurrentPreset } = usePresetStore();
  
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [deletePresetId, setDeletePresetId] = useState<string | null>(null);
  
  const [newPresetName, setNewPresetName] = useState("");
  const [newPresetDescription, setNewPresetDescription] = useState("");

  const handleCreatePreset = () => {
    if (!newPresetName.trim()) return;

    const now = new Date().toISOString();
    const newPreset: Preset = {
      id: Date.now().toString(),
      name: newPresetName,
      description: newPresetDescription,
      created_at: now,
      updated_at: now,
      channel_a: {
        enabled: true,
        min_power: 0,
        max_power: 50,
      },
      channel_b: {
        enabled: true,
        min_power: 0,
        max_power: 50,
      },
      settings: {},
    };

    addPreset(newPreset);
    setIsCreateDialogOpen(false);
    setNewPresetName("");
    setNewPresetDescription("");
  };

  const handleLoadPreset = (preset: Preset) => {
    setCurrentPreset(preset);
    navigate("/control");
  };

  const handleDeletePreset = (id: string) => {
    deletePreset(id);
    setDeletePresetId(null);
  };

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate("/")}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div className="flex-1">
          <h1 className="text-3xl font-bold tracking-tight">预设管理</h1>
          <p className="text-muted-foreground">保存和加载设备配置预设</p>
        </div>
        <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="mr-2 h-4 w-4" />
              新建预设
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>创建新预设</DialogTitle>
              <DialogDescription>
                输入预设名称和描述，将当前配置保存为预设
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="preset-name">预设名称</Label>
                <Input
                  id="preset-name"
                  placeholder="例如：舒适模式"
                  value={newPresetName}
                  onChange={(e) => setNewPresetName(e.target.value)}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="preset-description">描述（可选）</Label>
                <Input
                  id="preset-description"
                  placeholder="简要描述这个预设的用途"
                  value={newPresetDescription}
                  onChange={(e) => setNewPresetDescription(e.target.value)}
                />
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setIsCreateDialogOpen(false)}>
                取消
              </Button>
              <Button onClick={handleCreatePreset} disabled={!newPresetName.trim()}>
                <Save className="mr-2 h-4 w-4" />
                保存
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <Separator />

      {/* Preset List */}
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-semibold">我的预设 ({presets.length})</h2>
        </div>

        {presets.length === 0 ? (
          <Card>
            <CardContent className="pt-6 pb-6 text-center">
              <Save className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
              <p className="text-sm text-muted-foreground mb-4">
                还没有保存的预设
              </p>
              <Button onClick={() => setIsCreateDialogOpen(true)}>
                <Plus className="mr-2 h-4 w-4" />
                创建第一个预设
              </Button>
            </CardContent>
          </Card>
        ) : (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {presets.map((preset) => (
              <Card key={preset.id} className="hover:bg-accent/50 transition-colors">
                <CardHeader>
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <CardTitle className="text-lg">{preset.name}</CardTitle>
                      <CardDescription className="mt-1">
                        {preset.description || "无描述"}
                      </CardDescription>
                    </div>
                  </div>
                </CardHeader>
                <CardContent>
                  <div className="space-y-3">
                    {/* Channel Info */}
                    <div className="grid grid-cols-2 gap-2 text-sm">
                      <div className="space-y-1">
                        <p className="font-medium">通道 A</p>
                        <Badge variant="outline">{preset.channel_a.min_power}-{preset.channel_a.max_power}</Badge>
                      </div>
                      <div className="space-y-1">
                        <p className="font-medium">通道 B</p>
                        <Badge variant="outline">{preset.channel_b.min_power}-{preset.channel_b.max_power}</Badge>
                      </div>
                    </div>

                    <Separator />

                    {/* Actions */}
                    <div className="flex gap-2">
                      <Button
                        size="sm"
                        className="flex-1"
                        onClick={() => handleLoadPreset(preset)}
                      >
                        <Play className="mr-1 h-3 w-3" />
                        加载
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => setDeletePresetId(preset.id)}
                      >
                        <Trash2 className="h-3 w-3" />
                      </Button>
                    </div>

                    {/* Created Date */}
                    <p className="text-xs text-muted-foreground">
                      创建于 {new Date(preset.created_at).toLocaleDateString("zh-CN")}
                    </p>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Delete Confirmation Dialog */}
      <AlertDialog open={deletePresetId !== null} onOpenChange={(open) => !open && setDeletePresetId(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>确认删除</AlertDialogTitle>
            <AlertDialogDescription>
              确定要删除这个预设吗？此操作无法撤销。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => deletePresetId && handleDeletePreset(deletePresetId)}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              删除
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
