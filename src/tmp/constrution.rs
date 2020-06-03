struct conyard {
  x : f64,
  Y : f64,
  mvspd : f64,
  trans : bool false
 }
struct transconyard {
  Trans : true,
  X : f64,
  Y : f64,
  Creep :  i64,
  Power :  u32,
  Barrack : bool false,
  Refinery : bool false, 
  WarFactory : bool false,
  Defense : bool true,
  AirBase: bool false,
  NavalYard : bool false, 
  Radar : bool false,
  RepairFactory : bool false,
  University : bool false,
  OtherDevices : bool false,
  OtherDevice2 : bool false,
  OtherBuildDevice : bool false, 
}
struct Defensive {
  Power : true, 
  Wall : true,
  GDef : false,
  AADef :  false,
  AAAdef : false, 
  AGdef : false,
  ABdef : false,
}
struct SuperWeapons {
   University :true,
   SuperWeapon : true,

}
struct BarrackUnits {
  barrack : true,
  Basic_unit : true,
  Engineer : true,
  AnimalUnit : true, 
  SleathUnit : bool false,
  UpgradeBasicUnit : bool false,
  DemolutionUnit : bool false,

}

Universityruct Refinery{
  Refinery,true,
  harvester,true,
  Warfactory,true,
  AirBase,true,
  Radar, true,

  
}
struct WarfactoryUnits {
  WarFactory : true, 
  Basic_unit :true,
  harvester : true,
  construction_unit: true,
  RadarWarFactoryUnits:false,
  RepairFactoryUnit,false,
  university_Units:false,
}
struct AirBaseUnits {
   AirBase  : true,
   university_Units: false,
 
}
struct NavalYardUnits {
  NavalYard : true,
  BacisNavalUnit : true, 
 
}
struct powerBuilt_allied{
  power: u32
  Refinery : true,
  barrack : true
}
struct powerBuilt_like{
  power: u32
  Refinery : true,
  barrack : true
}
stuct powerBuilt_like{
  power: 150,
  Refinery : true,
  barrack : true,
}

impl powerBuilt_allied{
  power: 100,
  Refinery 

}
