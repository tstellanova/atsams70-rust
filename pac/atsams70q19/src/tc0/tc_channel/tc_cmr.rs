#[doc = "Reader of register TC_CMR"]
pub type R = crate::R<u32, super::TC_CMR>;
#[doc = "Writer for register TC_CMR"]
pub type W = crate::W<u32, super::TC_CMR>;
#[doc = "Register TC_CMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_CMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCLKS_A {
    #[doc = "0: Clock selected: internal PCK6 clock signal (from PMC)"]
    TIMER_CLOCK1 = 0,
    #[doc = "1: Clock selected: internal MCK/8 clock signal (from PMC)"]
    TIMER_CLOCK2 = 1,
    #[doc = "2: Clock selected: internal MCK/32 clock signal (from PMC)"]
    TIMER_CLOCK3 = 2,
    #[doc = "3: Clock selected: internal MCK/128 clock signal (from PMC)"]
    TIMER_CLOCK4 = 3,
    #[doc = "4: Clock selected: internal SLCK clock signal (from PMC)"]
    TIMER_CLOCK5 = 4,
    #[doc = "5: Clock selected: XC0"]
    XC0 = 5,
    #[doc = "6: Clock selected: XC1"]
    XC1 = 6,
    #[doc = "7: Clock selected: XC2"]
    XC2 = 7,
}
impl From<TCCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCCLKS`"]
pub type TCCLKS_R = crate::R<u8, TCCLKS_A>;
impl TCCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKS_A {
        match self.bits {
            0 => TCCLKS_A::TIMER_CLOCK1,
            1 => TCCLKS_A::TIMER_CLOCK2,
            2 => TCCLKS_A::TIMER_CLOCK3,
            3 => TCCLKS_A::TIMER_CLOCK4,
            4 => TCCLKS_A::TIMER_CLOCK5,
            5 => TCCLKS_A::XC0,
            6 => TCCLKS_A::XC1,
            7 => TCCLKS_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKS_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKS_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKS_A::XC2
    }
}
#[doc = "Write proxy for field `TCCLKS`"]
pub struct TCCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CLKI`"]
pub type CLKI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKI`"]
pub struct CLKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    XC0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    XC1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    XC2 = 3,
}
impl From<BURST_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<u8, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::NONE,
            1 => BURST_A::XC0,
            2 => BURST_A::XC1,
            3 => BURST_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BURST_A::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == BURST_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == BURST_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == BURST_A::XC2
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BURST_A::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURST_A::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURST_A::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURST_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LDBSTOP`"]
pub type LDBSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDBSTOP`"]
pub struct LDBSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBSTOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LDBDIS`"]
pub type LDBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDBDIS`"]
pub struct LDBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRGEDG_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Each edge"]
    EDGE = 3,
}
impl From<ETRGEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETRGEDG`"]
pub type ETRGEDG_R = crate::R<u8, ETRGEDG_A>;
impl ETRGEDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDG_A {
        match self.bits {
            0 => ETRGEDG_A::NONE,
            1 => ETRGEDG_A::RISING,
            2 => ETRGEDG_A::FALLING,
            3 => ETRGEDG_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDG_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDG_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDG_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDG_A::EDGE
    }
}
#[doc = "Write proxy for field `ETRGEDG`"]
pub struct ETRGEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGEDG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDG_A::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDG_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDG_A::FALLING)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ABETRG`"]
pub type ABETRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABETRG`"]
pub struct ABETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ABETRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CPCTRG`"]
pub type CPCTRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPCTRG`"]
pub struct CPCTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCTRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "RA Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRA_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRA_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDRA`"]
pub type LDRA_R = crate::R<u8, LDRA_A>;
impl LDRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRA_A {
        match self.bits {
            0 => LDRA_A::NONE,
            1 => LDRA_A::RISING,
            2 => LDRA_A::FALLING,
            3 => LDRA_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRA_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRA_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRA_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRA_A::EDGE
    }
}
#[doc = "Write proxy for field `LDRA`"]
pub struct LDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRA_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRA_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRA_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRA_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "RB Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRB_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRB_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDRB`"]
pub type LDRB_R = crate::R<u8, LDRB_A>;
impl LDRB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRB_A {
        match self.bits {
            0 => LDRB_A::NONE,
            1 => LDRB_A::RISING,
            2 => LDRB_A::FALLING,
            3 => LDRB_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRB_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRB_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRB_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRB_A::EDGE
    }
}
#[doc = "Write proxy for field `LDRB`"]
pub struct LDRB_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRB_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRB_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRB_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRB_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Loading Edge Subsampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SBSMPLR_A {
    #[doc = "0: Load a Capture Register each selected edge"]
    ONE = 0,
    #[doc = "1: Load a Capture Register every 2 selected edges"]
    HALF = 1,
    #[doc = "2: Load a Capture Register every 4 selected edges"]
    FOURTH = 2,
    #[doc = "3: Load a Capture Register every 8 selected edges"]
    EIGHTH = 3,
    #[doc = "4: Load a Capture Register every 16 selected edges"]
    SIXTEENTH = 4,
}
impl From<SBSMPLR_A> for u8 {
    #[inline(always)]
    fn from(variant: SBSMPLR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SBSMPLR`"]
pub type SBSMPLR_R = crate::R<u8, SBSMPLR_A>;
impl SBSMPLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SBSMPLR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SBSMPLR_A::ONE),
            1 => Val(SBSMPLR_A::HALF),
            2 => Val(SBSMPLR_A::FOURTH),
            3 => Val(SBSMPLR_A::EIGHTH),
            4 => Val(SBSMPLR_A::SIXTEENTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBSMPLR_A::ONE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == SBSMPLR_A::HALF
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == SBSMPLR_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == SBSMPLR_A::EIGHTH
    }
    #[doc = "Checks if the value of the field is `SIXTEENTH`"]
    #[inline(always)]
    pub fn is_sixteenth(&self) -> bool {
        *self == SBSMPLR_A::SIXTEENTH
    }
}
#[doc = "Write proxy for field `SBSMPLR`"]
pub struct SBSMPLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBSMPLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBSMPLR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBSMPLR_A::ONE)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(SBSMPLR_A::HALF)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::FOURTH)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::EIGHTH)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn sixteenth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::SIXTEENTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&self) -> SBSMPLR_R {
        SBSMPLR_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TCCLKS_W {
        TCCLKS_W { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> CLKI_W {
        CLKI_W { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> LDBSTOP_W {
        LDBSTOP_W { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> LDBDIS_W {
        LDBDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> ETRGEDG_W {
        ETRGEDG_W { w: self }
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> ABETRG_W {
        ABETRG_W { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> CPCTRG_W {
        CPCTRG_W { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> LDRA_W {
        LDRA_W { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> LDRB_W {
        LDRB_W { w: self }
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&mut self) -> SBSMPLR_W {
        SBSMPLR_W { w: self }
    }
}
