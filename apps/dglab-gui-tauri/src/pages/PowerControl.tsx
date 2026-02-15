import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useDeviceStore } from "@/stores/deviceStore";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { ArrowLeft, Play, Square, AlertCircle, Zap, Wand2 } from "lucide-react";

export function PowerControl() {
  const navigate = useNavigate();
  const {
    currentDevice,
    isConnected,
    powerA,
    powerB,
    setPower,
    startDevice,
    stopDevice,
    emergencyStop,
  } = useDeviceStore();

  const [localPowerA, setLocalPowerA] = useState(powerA);
  const [localPowerB, setLocalPowerB] = useState(powerB);
  const [isRunning, setIsRunning] = useState(false);

  useEffect(() => {
    setLocalPowerA(powerA);
    setLocalPowerB(powerB);
  }, [powerA, powerB]);

  // Redirect if not connected
  useEffect(() => {
    if (!isConnected) {
      navigate("/scanner");
    }
  }, [isConnected, navigate]);

  const handlePowerChangeA = async (value: number[]) => {
    const newValue = value[0];
    setLocalPowerA(newValue);
    try {
      await setPower("A", newValue);
    } catch (error) {
      console.error("Failed to set power A:", error);
    }
  };

  const handlePowerChangeB = async (value: number[]) => {
    const newValue = value[0];
    setLocalPowerB(newValue);
    try {
      await setPower("B", newValue);
    } catch (error) {
      console.error("Failed to set power B:", error);
    }
  };

  const handleStart = async () => {
    try {
      await startDevice();
      setIsRunning(true);
    } catch (error) {
      console.error("Failed to start device:", error);
    }
  };

  const handleStop = async () => {
    try {
      await stopDevice();
      setIsRunning(false);
    } catch (error) {
      console.error("Failed to stop device:", error);
    }
  };

  const handleEmergencyStop = async () => {
    try {
      await emergencyStop();
      setIsRunning(false);
      setLocalPowerA(0);
      setLocalPowerB(0);
    } catch (error) {
      console.error("Emergency stop failed:", error);
    }
  };

  if (!isConnected || !currentDevice) {
    return null;
  }

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate("/")}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div className="flex-1">
          <h1 className="text-3xl font-bold tracking-tight">功率控制</h1>
          <p className="text-muted-foreground">{currentDevice.name}</p>
        </div>
        <Badge variant={isRunning ? "default" : "secondary"}>
          {isRunning ? "运行中" : "已停止"}
        </Badge>
      </div>

      <Separator />

      {/* Power Control Cards */}
      <div className="grid gap-6 md:grid-cols-2">
        {/* Channel A */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap className="h-5 w-5 text-blue-500" />
              通道 A
            </CardTitle>
            <CardDescription>调节通道 A 的输出功率</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">功率值</span>
                <Badge variant="outline" className="text-lg font-bold">
                  {localPowerA}
                </Badge>
              </div>
              <Slider
                value={[localPowerA]}
                onValueChange={handlePowerChangeA}
                max={200}
                step={1}
                disabled={!isConnected}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-muted-foreground">
                <span>0</span>
                <span>100</span>
                <span>200</span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Channel B */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap className="h-5 w-5 text-purple-500" />
              通道 B
            </CardTitle>
            <CardDescription>调节通道 B 的输出功率</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">功率值</span>
                <Badge variant="outline" className="text-lg font-bold">
                  {localPowerB}
                </Badge>
              </div>
              <Slider
                value={[localPowerB]}
                onValueChange={handlePowerChangeB}
                max={200}
                step={1}
                disabled={!isConnected}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-muted-foreground">
                <span>0</span>
                <span>100</span>
                <span>200</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Control Buttons */}
      <Card>
        <CardHeader>
          <CardTitle>设备控制</CardTitle>
          <CardDescription>启动或停止设备输出</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-4">
            <Button
              onClick={handleStart}
              disabled={isRunning || !isConnected}
              size="lg"
              className="h-16"
            >
              <Play className="mr-2 h-5 w-5" />
              启动
            </Button>
            <Button
              onClick={handleStop}
              disabled={!isRunning || !isConnected}
              variant="secondary"
              size="lg"
              className="h-16"
            >
              <Square className="mr-2 h-5 w-5" />
              停止
            </Button>
            <Button
              onClick={handleEmergencyStop}
              disabled={!isConnected}
              variant="destructive"
              size="lg"
              className="h-16"
            >
              <AlertCircle className="mr-2 h-5 w-5" />
              紧急停止
            </Button>
            <Button
              onClick={() => navigate("/waveform")}
              variant="outline"
              size="lg"
              className="h-16"
            >
              <Wand2 className="mr-2 h-5 w-5" />
              波形设置
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Safety Warning */}
      <Card className="border-yellow-500/50 bg-yellow-50 dark:bg-yellow-950/20">
        <CardContent className="pt-6">
          <div className="flex gap-3">
            <AlertCircle className="h-5 w-5 text-yellow-600 dark:text-yellow-500 flex-shrink-0 mt-0.5" />
            <div className="space-y-1">
              <p className="text-sm font-medium text-yellow-800 dark:text-yellow-200">
                安全提示
              </p>
              <p className="text-sm text-yellow-700 dark:text-yellow-300">
                请在使用前仔细阅读产品说明书，了解安全使用方法。如遇紧急情况，请立即按下紧急停止按钮。
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
