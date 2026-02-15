import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useWaveformStore } from "@/stores/waveformStore";
import { WaveformType } from "@/types/waveform";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Badge } from "@/components/ui/badge";
import { ArrowLeft, Wand2, Save } from "lucide-react";

export function WaveformGenerator() {
  const navigate = useNavigate();
  const { waveformA, waveformB, setWaveform } = useWaveformStore();
  
  const [selectedChannel, setSelectedChannel] = useState<"A" | "B">("A");
  const currentWaveform = selectedChannel === "A" ? waveformA : waveformB;
  
  const [waveformType, setWaveformType] = useState(currentWaveform.params.waveform_type);
  const [frequency, setFrequency] = useState(currentWaveform.params.frequency);
  const [pulseWidth, setPulseWidth] = useState(currentWaveform.params.pulse_width);
  const [minPower, setMinPower] = useState(currentWaveform.params.min_power);
  const [maxPower, setMaxPower] = useState(currentWaveform.params.max_power);
  const [periodMs, setPeriodMs] = useState(currentWaveform.params.period_ms);
  const [dutyCycle, setDutyCycle] = useState(currentWaveform.params.duty_cycle);

  const handleApply = () => {
    const newWaveform = {
      ...currentWaveform,
      params: {
        waveform_type: waveformType,
        frequency,
        pulse_width: pulseWidth,
        min_power: minPower,
        max_power: maxPower,
        period_ms: periodMs,
        duty_cycle: dutyCycle,
      },
    };
    
    setWaveform(selectedChannel, newWaveform);
  };

  const waveformTypes = [
    { value: WaveformType.Continuous, label: "连续波" },
    { value: WaveformType.Pulse, label: "脉冲波" },
    { value: WaveformType.Sine, label: "正弦波" },
    { value: WaveformType.Square, label: "方波" },
    { value: WaveformType.Triangle, label: "三角波" },
    { value: WaveformType.Sawtooth, label: "锯齿波" },
    { value: WaveformType.Breathing, label: "呼吸波" },
    { value: WaveformType.Fade, label: "渐强渐弱" },
  ];

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="icon" onClick={() => navigate("/control")}>
          <ArrowLeft className="h-5 w-5" />
        </Button>
        <div className="flex-1">
          <h1 className="text-3xl font-bold tracking-tight">波形生成器</h1>
          <p className="text-muted-foreground">自定义输出波形参数</p>
        </div>
      </div>

      <Separator />

      {/* Channel Selector */}
      <div className="flex gap-2">
        <Button
          variant={selectedChannel === "A" ? "default" : "outline"}
          onClick={() => setSelectedChannel("A")}
          className="flex-1"
        >
          通道 A
        </Button>
        <Button
          variant={selectedChannel === "B" ? "default" : "outline"}
          onClick={() => setSelectedChannel("B")}
          className="flex-1"
        >
          通道 B
        </Button>
      </div>

      {/* Waveform Type Selection */}
      <Card>
        <CardHeader>
          <CardTitle>波形类型</CardTitle>
          <CardDescription>选择输出波形的类型</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label>波形</Label>
            <Select value={waveformType} onValueChange={(value) => setWaveformType(value as WaveformType)}>
              <SelectTrigger>
                <SelectValue placeholder="选择波形类型" />
              </SelectTrigger>
              <SelectContent>
                {waveformTypes.map((type) => (
                  <SelectItem key={type.value} value={type.value}>
                    {type.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Basic Parameters */}
      <Card>
        <CardHeader>
          <CardTitle>基本参数</CardTitle>
          <CardDescription>调节波形的基本属性</CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Frequency */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>频率 (Hz)</Label>
              <Badge variant="outline">{frequency}</Badge>
            </div>
            <Slider
              value={[frequency]}
              onValueChange={(v) => setFrequency(v[0])}
              min={1}
              max={500}
              step={1}
            />
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>1 Hz</span>
              <span>500 Hz</span>
            </div>
          </div>

          {/* Pulse Width */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>脉宽 (μs)</Label>
              <Badge variant="outline">{pulseWidth}</Badge>
            </div>
            <Slider
              value={[pulseWidth]}
              onValueChange={(v) => setPulseWidth(v[0])}
              min={50}
              max={1000}
              step={10}
            />
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>50 μs</span>
              <span>1000 μs</span>
            </div>
          </div>

          {/* Duty Cycle */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>占空比 (%)</Label>
              <Badge variant="outline">{dutyCycle}</Badge>
            </div>
            <Slider
              value={[dutyCycle]}
              onValueChange={(v) => setDutyCycle(v[0])}
              min={0}
              max={100}
              step={1}
            />
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>0%</span>
              <span>100%</span>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Power Range */}
      <Card>
        <CardHeader>
          <CardTitle>功率范围</CardTitle>
          <CardDescription>设置波形的最小和最大功率</CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Min Power */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>最小功率</Label>
              <Badge variant="outline">{minPower}</Badge>
            </div>
            <Slider
              value={[minPower]}
              onValueChange={(v) => setMinPower(v[0])}
              min={0}
              max={200}
              step={1}
            />
          </div>

          {/* Max Power */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>最大功率</Label>
              <Badge variant="outline">{maxPower}</Badge>
            </div>
            <Slider
              value={[maxPower]}
              onValueChange={(v) => setMaxPower(v[0])}
              min={0}
              max={200}
              step={1}
            />
          </div>
        </CardContent>
      </Card>

      {/* Period */}
      <Card>
        <CardHeader>
          <CardTitle>周期设置</CardTitle>
          <CardDescription>设置波形的完整周期时间</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>周期 (ms)</Label>
              <Badge variant="outline">{periodMs}</Badge>
            </div>
            <Slider
              value={[periodMs]}
              onValueChange={(v) => setPeriodMs(v[0])}
              min={100}
              max={10000}
              step={100}
            />
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>0.1 s</span>
              <span>10 s</span>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Action Buttons */}
      <div className="grid gap-4 md:grid-cols-2">
        <Button onClick={handleApply} size="lg" className="h-14">
          <Wand2 className="mr-2 h-5 w-5" />
          应用到通道 {selectedChannel}
        </Button>
        <Button variant="outline" size="lg" className="h-14">
          <Save className="mr-2 h-5 w-5" />
          保存为预设
        </Button>
      </div>
    </div>
  );
}
