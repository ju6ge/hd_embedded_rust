#[doc = "Writer for register EEFC_FCR"]
pub type W = crate::W<u32, super::EEFC_FCR>;
#[doc = "Register EEFC_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EEFC_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMD_AW {
    #[doc = "0: Get Flash descriptor"]
    GETD,
    #[doc = "1: Write page"]
    WP,
    #[doc = "2: Write page and lock"]
    WPL,
    #[doc = "3: Erase page and write page"]
    EWP,
    #[doc = "4: Erase page and write page then lock"]
    EWPL,
    #[doc = "5: Erase all"]
    EA,
    #[doc = "7: Erase pages"]
    EPA,
    #[doc = "8: Set lock bit"]
    SLB,
    #[doc = "9: Clear lock bit"]
    CLB,
    #[doc = "10: Get lock bit"]
    GLB,
    #[doc = "11: Set GPNVM bit"]
    SGPB,
    #[doc = "12: Clear GPNVM bit"]
    CGPB,
    #[doc = "13: Get GPNVM bit"]
    GGPB,
    #[doc = "14: Start read unique identifier"]
    STUI,
    #[doc = "15: Stop read unique identifier"]
    SPUI,
    #[doc = "16: Get CALIB bit"]
    GCALB,
    #[doc = "17: Erase sector"]
    ES,
    #[doc = "18: Write user signature"]
    WUS,
    #[doc = "19: Erase user signature"]
    EUS,
    #[doc = "20: Start read user signature"]
    STUS,
    #[doc = "21: Stop read user signature"]
    SPUS,
}
impl From<FCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: FCMD_AW) -> Self {
        match variant {
            FCMD_AW::GETD => 0,
            FCMD_AW::WP => 1,
            FCMD_AW::WPL => 2,
            FCMD_AW::EWP => 3,
            FCMD_AW::EWPL => 4,
            FCMD_AW::EA => 5,
            FCMD_AW::EPA => 7,
            FCMD_AW::SLB => 8,
            FCMD_AW::CLB => 9,
            FCMD_AW::GLB => 10,
            FCMD_AW::SGPB => 11,
            FCMD_AW::CGPB => 12,
            FCMD_AW::GGPB => 13,
            FCMD_AW::STUI => 14,
            FCMD_AW::SPUI => 15,
            FCMD_AW::GCALB => 16,
            FCMD_AW::ES => 17,
            FCMD_AW::WUS => 18,
            FCMD_AW::EUS => 19,
            FCMD_AW::STUS => 20,
            FCMD_AW::SPUS => 21,
        }
    }
}
#[doc = "Write proxy for field `FCMD`"]
pub struct FCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMD_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Get Flash descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMD_AW::GETD)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMD_AW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMD_AW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMD_AW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMD_AW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMD_AW::EA)
    }
    #[doc = "Erase pages"]
    #[inline(always)]
    pub fn epa(self) -> &'a mut W {
        self.variant(FCMD_AW::EPA)
    }
    #[doc = "Set lock bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMD_AW::SLB)
    }
    #[doc = "Clear lock bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMD_AW::CLB)
    }
    #[doc = "Get lock bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMD_AW::GLB)
    }
    #[doc = "Set GPNVM bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::SGPB)
    }
    #[doc = "Clear GPNVM bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::CGPB)
    }
    #[doc = "Get GPNVM bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMD_AW::GGPB)
    }
    #[doc = "Start read unique identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMD_AW::STUI)
    }
    #[doc = "Stop read unique identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMD_AW::SPUI)
    }
    #[doc = "Get CALIB bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMD_AW::GCALB)
    }
    #[doc = "Erase sector"]
    #[inline(always)]
    pub fn es(self) -> &'a mut W {
        self.variant(FCMD_AW::ES)
    }
    #[doc = "Write user signature"]
    #[inline(always)]
    pub fn wus(self) -> &'a mut W {
        self.variant(FCMD_AW::WUS)
    }
    #[doc = "Erase user signature"]
    #[inline(always)]
    pub fn eus(self) -> &'a mut W {
        self.variant(FCMD_AW::EUS)
    }
    #[doc = "Start read user signature"]
    #[inline(always)]
    pub fn stus(self) -> &'a mut W {
        self.variant(FCMD_AW::STUS)
    }
    #[doc = "Stop read user signature"]
    #[inline(always)]
    pub fn spus(self) -> &'a mut W {
        self.variant(FCMD_AW::SPUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `FARG`"]
pub struct FARG_W<'a> {
    w: &'a mut W,
}
impl<'a> FARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Flash Writing Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FKEY_AW {
    #[doc = "90: The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD,
}
impl From<FKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: FKEY_AW) -> Self {
        match variant {
            FKEY_AW::PASSWD => 90,
        }
    }
}
#[doc = "Write proxy for field `FKEY`"]
pub struct FKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> FKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FKEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    pub fn fcmd(&mut self) -> FCMD_W {
        FCMD_W { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    pub fn farg(&mut self) -> FARG_W {
        FARG_W { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    pub fn fkey(&mut self) -> FKEY_W {
        FKEY_W { w: self }
    }
}
