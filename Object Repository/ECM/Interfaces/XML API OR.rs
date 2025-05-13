<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>XML API OR</name>
   <tag></tag>
   <elementGuidId>ace66f43-8be4-42f4-a97a-fc1ea85cce01</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cSMSRequest\u003e\n  \u003cCampaign\u003e\n    \u003cCampaignName\u003eTest Campaign with XML\u003c/CampaignName\u003e\n    \u003cDesc\u003eThis is the description for campaign\u003c/Desc\u003e\n    \u003cJobType\u003e1\u003c/JobType\u003e\n    \u003cInterfaceType\u003e5.3\u003c/InterfaceType\u003e\n    \u003cprotocolId\u003e\u003c/protocolId\u003e\n  \u003c/Campaign\u003e\n  \u003cSender\u003e\n    \u003cUsername\u003eaabaker\u003c/Username\u003e\n    \u003cPassword\u003eYousif@123\u003c/Password\u003e\n    \u003csenderAddr\u003eengageX\u003c/senderAddr\u003e\n  \u003c/Sender\u003e\n  \u003cRecipient\u003e\n    \u003cNumber\u003e971554573936\u003c/Number\u003e\n    \u003cDistributionListID\u003e\u003c/DistributionListID\u003e\n    \u003cSegmentList\u003e\u003c/SegmentList\u003e\n    \u003cBitSetUrl\u003e\u003c/BitSetUrl\u003e\n    \u003ccontactGroupId\u003e\u003c/contactGroupId\u003e\n  \u003c/Recipient\u003e\n  \u003cMsgDetails\u003e\n    \u003cMsg\u003eEngageX Health Check: ECM XML API Test\u003c/Msg\u003e\n    \u003cMsgCategory\u003e4.5\u003c/MsgCategory\u003e\n    \u003cChannel\u003e2.1\u003c/Channel\u003e\n    \u003cMsgContentType\u003e3.1\u003c/MsgContentType\u003e\n    \u003cDNDCategory\u003eCampaign\u003c/DNDCategory\u003e\n    \u003cScheduledTime\u003e\u003c/ScheduledTime\u003e\n    \u003cDefLang\u003ear\u003c/DefLang\u003e\n  \u003c/MsgDetails\u003e\n  \u003cDeliveryReport\u003e\n    \u003cReportEnabled\u003etrue\u003c/ReportEnabled\u003e\n  \u003c/DeliveryReport\u003e\n\u003c/SMSRequest\u003e&quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
      <webElementGuid>0d3a2589-8131-49cd-8e3e-68b7a7befdcf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://smartmessaging.etisalat.ae:9096/mmsReceiver/submitSMS</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
