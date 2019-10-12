#[doc = "Reader of register MCAN_PSR"]
pub type R = crate::R<u32, super::MCAN_PSR>;
#[doc = "Last Error Code (set to 111 on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEC_A {
    #[doc = "0: No error occurred since LEC has been reset by successful reception or transmission."]
    NO_ERROR,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF_ERROR,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    FORM_ERROR,
    #[doc = "3: The message transmitted by the MCAN was not acknowledged by another node."]
    ACK_ERROR,
    #[doc = "4: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant."]
    BIT1_ERROR,
    #[doc = "5: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value '0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the processor to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0_ERROR,
    #[doc = "6: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    CRC_ERROR,
    #[doc = "7: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last processor read access to the Protocol Status Register."]
    NO_CHANGE,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        match variant {
            LEC_A::NO_ERROR => 0,
            LEC_A::STUFF_ERROR => 1,
            LEC_A::FORM_ERROR => 2,
            LEC_A::ACK_ERROR => 3,
            LEC_A::BIT1_ERROR => 4,
            LEC_A::BIT0_ERROR => 5,
            LEC_A::CRC_ERROR => 6,
            LEC_A::NO_CHANGE => 7,
        }
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NO_ERROR,
            1 => LEC_A::STUFF_ERROR,
            2 => LEC_A::FORM_ERROR,
            3 => LEC_A::ACK_ERROR,
            4 => LEC_A::BIT1_ERROR,
            5 => LEC_A::BIT0_ERROR,
            6 => LEC_A::CRC_ERROR,
            7 => LEC_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == LEC_A::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == LEC_A::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `ACK_ERROR`"]
    #[inline(always)]
    pub fn is_ack_error(&self) -> bool {
        *self == LEC_A::ACK_ERROR
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR`"]
    #[inline(always)]
    pub fn is_bit1_error(&self) -> bool {
        *self == LEC_A::BIT1_ERROR
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR`"]
    #[inline(always)]
    pub fn is_bit0_error(&self) -> bool {
        *self == LEC_A::BIT0_ERROR
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR`"]
    #[inline(always)]
    pub fn is_crc_error(&self) -> bool {
        *self == LEC_A::CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == LEC_A::NO_CHANGE
    }
}
#[doc = "Activity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACT_A {
    #[doc = "0: Node is synchronizing on CAN communication"]
    SYNCHRONIZING,
    #[doc = "1: Node is neither receiver nor transmitter"]
    IDLE,
    #[doc = "2: Node is operating as receiver"]
    RECEIVER,
    #[doc = "3: Node is operating as transmitter"]
    TRANSMITTER,
}
impl From<ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        match variant {
            ACT_A::SYNCHRONIZING => 0,
            ACT_A::IDLE => 1,
            ACT_A::RECEIVER => 2,
            ACT_A::TRANSMITTER => 3,
        }
    }
}
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<u8, ACT_A>;
impl ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            0 => ACT_A::SYNCHRONIZING,
            1 => ACT_A::IDLE,
            2 => ACT_A::RECEIVER,
            3 => ACT_A::TRANSMITTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZING`"]
    #[inline(always)]
    pub fn is_synchronizing(&self) -> bool {
        *self == ACT_A::SYNCHRONIZING
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ACT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVER`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == ACT_A::RECEIVER
    }
    #[doc = "Checks if the value of the field is `TRANSMITTER`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == ACT_A::TRANSMITTER
    }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, bool>;
#[doc = "Reader of field `BO`"]
pub type BO_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLEC`"]
pub type FLEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RESI`"]
pub type RESI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBRS`"]
pub type RBRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `REDL`"]
pub type REDL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Fast Last Error Code (set to 111 on read)"]
    #[inline(always)]
    pub fn flec(&self) -> FLEC_R {
        FLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - ESI Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRS Flag of Last Received CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message (cleared on read)"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
